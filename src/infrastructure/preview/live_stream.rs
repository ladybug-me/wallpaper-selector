use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, sync_channel};
use std::time::Instant;

use futures_channel::mpsc::UnboundedSender;

use crate::contracts::preview::BufPool;
use crate::infrastructure::runtime::Wake;

const DECODER_ENV: &str = "SKWD_LIVE_PREVIEW_DECODER";
const THREADS_ENV: &str = "SKWD_LIVE_PREVIEW_THREADS";

fn scanner_bin() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|executable| {
            executable.parent().map(|directory| directory.join("skwd-wall-scan"))
        })
        .filter(|path| path.exists())
        .unwrap_or_else(|| PathBuf::from("skwd-wall-scan"))
}

pub struct LiveStreamer {
    child: Child,
    stdin: ChildStdin,
    rx: Receiver<(u32, Vec<u8>)>,
    stop: Arc<AtomicBool>,
    pool: Arc<BufPool>,
    current_path: Option<String>,
    token: u32,
    pub last_active: Instant,
}

impl LiveStreamer {
    pub fn start(
        wake: UnboundedSender<Wake>,
        now: Instant,
        pool: Arc<BufPool>,
        fps: u32,
    ) -> Option<Self> {
        let decoder = std::env::var(DECODER_ENV).unwrap_or_else(|_| String::from("software"));
        let threads = std::env::var(THREADS_ENV).unwrap_or_else(|_| String::from("1"));
        let mut child = Command::new(scanner_bin())
            .arg("--stream-persist")
            .env("SKWD_LIVE_PREVIEW_FPS", fps.to_string())
            .env(DECODER_ENV, &decoder)
            .env(THREADS_ENV, &threads)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        let stdin = child.stdin.take()?;
        let mut stdout = child.stdout.take()?;
        unsafe {
            use std::os::fd::AsRawFd;
            libc::fcntl(stdout.as_raw_fd(), libc::F_SETPIPE_SZ, 1_048_576);
        }
        let (tx, rx) = sync_channel::<(u32, Vec<u8>)>(2);
        let stop = Arc::new(AtomicBool::new(false));
        let stop_t = stop.clone();
        let pool_t = pool.clone();
        std::thread::Builder::new()
            .name("skwd-live".into())
            .spawn(move || {
                let mut head = [0u8; 4];
                while !stop_t.load(Ordering::Relaxed) {
                    let mut buf = pool_t.take();
                    if stdout
                        .read_exact(&mut head)
                        .and_then(|()| stdout.read_exact(&mut buf))
                        .is_err()
                    {
                        break;
                    }
                    if tx.send((u32::from_le_bytes(head), buf)).is_err() {
                        break;
                    }
                    let _ = wake.unbounded_send(Wake::Decoded);
                }
            })
            .ok();
        log::info!("live streamer: spawned persistent helper decoder={decoder} threads={threads}");
        Some(Self { child, stdin, rx, stop, pool, current_path: None, token: 0, last_active: now })
    }

    pub fn set_path(&mut self, path: &str) -> bool {
        if self.current_path.as_deref() == Some(path) {
            return true;
        }
        self.token = self.token.wrapping_add(1);
        if writeln!(self.stdin, "{path}\t{}", self.token).and_then(|()| self.stdin.flush()).is_err()
        {
            return false;
        }
        self.current_path = Some(path.to_string());
        true
    }

    pub fn pause(&mut self) {
        if self.current_path.is_none() {
            return;
        }
        let _ = writeln!(self.stdin).and_then(|()| self.stdin.flush());
        self.current_path = None;
    }

    pub fn latest_frame(&self) -> Option<Vec<u8>> {
        let mut frame: Option<Vec<u8>> = None;
        while let Ok((tok, buf)) = self.rx.try_recv() {
            if tok == self.token {
                if let Some(old) = frame.replace(buf) {
                    self.pool.put(old);
                }
            } else {
                self.pool.put(buf);
            }
        }
        frame
    }
}

impl Drop for LiveStreamer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
