use std::time::{Duration, Instant};

use log::info;

pub(super) const RING: usize = 240;

pub(super) struct Ring {
    buf: [f32; RING],
    pub(super) len: usize,
    pos: usize,
}

impl Ring {
    pub(super) const fn new() -> Self {
        Self { buf: [0.0; RING], len: 0, pos: 0 }
    }

    pub(super) fn push(&mut self, val: f32) {
        self.buf[self.pos] = val;
        self.pos = (self.pos + 1) % RING;
        self.len = (self.len + 1).min(RING);
    }

    pub(super) fn percentiles(&self, ps: [f32; 3]) -> [f32; 3] {
        if self.len == 0 {
            return [0.0; 3];
        }
        let mut sorted = [0.0f32; RING];
        let slice = &mut sorted[..self.len];
        slice.copy_from_slice(&self.buf[..self.len]);
        slice.sort_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap_or(std::cmp::Ordering::Equal));
        ps.map(|frac| slice[((slice.len() - 1) as f32 * frac) as usize])
    }
}

pub struct Metrics {
    pub log_enabled: bool,
    pub hud_enabled: bool,
    pub(super) frame_ms: Ring,
    pub(super) tick_ms: Ring,
    last_frame: Option<Instant>,
    pub(super) frames: u64,
    pub instances: usize,
    pub decoded: u64,
    pub decode_ms_avg: f32,
    pub decode_ms_max: f32,
    #[cfg(feature = "obs-heap")]
    prev_allocs: usize,
    allocs_per_frame: usize,
    last_report: Instant,
    started: Instant,
}

#[cfg(feature = "obs-heap")]
fn mem_summary(allocs_per_frame: usize) -> String {
    let mem = super::super::memory::mem_breakdown();
    format!(
        "heap {:.1} MB live (peak {:.1}) | rss {} / pss {} / dirty {} MB | alloc {}/frame",
        super::super::allocation::live_bytes() as f64 / 1e6,
        super::super::allocation::peak_bytes() as f64 / 1e6,
        mem.rss_kb / 1024,
        mem.pss_kb / 1024,
        mem.private_dirty_kb / 1024,
        allocs_per_frame,
    )
}

#[cfg(not(feature = "obs-heap"))]
fn mem_summary(_allocs_per_frame: usize) -> String {
    format!("rss {}", super::super::rss_label())
}

fn env_on(name: &str) -> bool {
    std::env::var(name).is_ok_and(|val| val == "1" || val == "true")
}

static FORCE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn force_on() {
    FORCE.store(true, std::sync::atomic::Ordering::Relaxed);
}

pub fn hud_active() -> bool {
    FORCE.load(std::sync::atomic::Ordering::Relaxed) || env_on("SKWD_WALL_HUD")
}

impl Metrics {
    pub fn new() -> Self {
        let forced = FORCE.load(std::sync::atomic::Ordering::Relaxed);
        let hud = env_on("SKWD_WALL_HUD");
        Self {
            log_enabled: forced || hud || env_on("SKWD_WALL_METRICS"),
            hud_enabled: hud || forced,
            frame_ms: Ring::new(),
            tick_ms: Ring::new(),
            last_frame: None,
            frames: 0,
            instances: 0,
            decoded: 0,
            decode_ms_avg: 0.0,
            decode_ms_max: 0.0,
            #[cfg(feature = "obs-heap")]
            prev_allocs: 0,
            allocs_per_frame: 0,
            last_report: Instant::now(),
            started: Instant::now(),
        }
    }

    pub fn on_frame(&mut self, now: Instant, tick_dur: Duration, instances: usize) {
        if let Some(last) = self.last_frame {
            let dt = now.duration_since(last).as_secs_f32() * 1000.0;
            if dt < 1000.0 {
                self.frame_ms.push(dt);
            }
        }
        self.last_frame = Some(now);
        self.frames += 1;
        self.tick_ms.push(tick_dur.as_secs_f32() * 1000.0);
        self.instances = instances;
        #[cfg(feature = "obs-heap")]
        {
            let count = super::super::allocation::alloc_count();
            self.allocs_per_frame = count.saturating_sub(self.prev_allocs);
            self.prev_allocs = count;
        }
        #[cfg(feature = "obs-tracy")]
        {
            tracy_client::plot!("tick_ms", tick_dur.as_secs_f64() * 1000.0);
            #[cfg(feature = "obs-heap")]
            {
                tracy_client::plot!(
                    "heap_live_mb",
                    super::super::allocation::live_bytes() as f64 / 1e6
                );
                tracy_client::plot!("allocs_per_frame", self.allocs_per_frame as f64);
            }
        }
        if self.log_enabled && self.last_report.elapsed() > Duration::from_secs(5) {
            self.last_report = Instant::now();
            info!("metrics: {}", self.report());
        }
    }

    pub fn note_decodes(&mut self, count: u64, avg_ms: f32, max_ms: f32) {
        self.decoded += count;
        self.decode_ms_avg = avg_ms;
        self.decode_ms_max = self.decode_ms_max.max(max_ms);
    }

    pub fn report(&self) -> String {
        let frame = self.frame_ms.percentiles([0.5, 0.95, 0.99]);
        let tick = self.tick_ms.percentiles([0.5, 0.95, 0.99]);
        format!(
            "frames {} | frame ms p50/p95/p99 {:.1}/{:.1}/{:.1} | tick ms {:.2}/{:.2}/{:.2} | instances {} | decoded {} (avg {:.1} ms, max {:.1} ms) | {} | up {:.0}s",
            self.frames,
            frame[0],
            frame[1],
            frame[2],
            tick[0],
            tick[1],
            tick[2],
            self.instances,
            self.decoded,
            self.decode_ms_avg,
            self.decode_ms_max,
            mem_summary(self.allocs_per_frame),
            self.started.elapsed().as_secs_f32(),
        )
    }

    pub fn hud_lines(&self) -> String {
        let frame = self.frame_ms.percentiles([0.5, 0.95, 0.99]);
        let tick = self.tick_ms.percentiles([0.5, 0.95, 0.99]);
        format!(
            "frame {:.1} / {:.1} / {:.1} ms\ntick  {:.2} / {:.2} / {:.2} ms\ninst {}  dec {} ({:.1} ms)\n{}",
            frame[0],
            frame[1],
            frame[2],
            tick[0],
            tick[1],
            tick[2],
            self.instances,
            self.decoded,
            self.decode_ms_avg,
            mem_summary(self.allocs_per_frame),
        )
    }

    pub fn exit_summary(&self) {
        info!("metrics summary: {}", self.report());
    }
}
