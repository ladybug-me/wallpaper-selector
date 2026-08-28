struct SandyUniform {
    center: vec2<f32>,
    resolution: vec2<f32>,
    hero: vec2<f32>,
    dir: f32,
    layer_a: f32,
    layer_b: f32,
    progress: f32,
    time: f32,
    vis: f32,
    seed: f32,
    strands: f32,
    twist: f32,
    orbit: f32,
    turbulence: f32,
    waist: f32,
    front: f32,
    fan: f32,
    carry: f32,
    layer_b2: f32,
    layer_b3: f32,
    bcut: f32,
    bmix: f32,
    arc: f32,
    swap_loop: f32,
    swirl: f32,
    wave_flag: f32,
    ring_spin: f32,
    ring_wave: f32,
    ring_soft: f32,
    grid: vec2<f32>,
    swap_style: f32,
    video_in: f32,
    video_out: f32,
    ring_size: f32,
    _pad0: f32,
    _pad1: f32,
}

@group(0) @binding(0) var<uniform> u: SandyUniform;
@group(0) @binding(1) var near_tex: texture_2d_array<f32>;
@group(0) @binding(2) var samp: sampler;
@group(0) @binding(3) var prev_tex: texture_2d<f32>;
@group(0) @binding(4) var prev_out_tex: texture_2d<f32>;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) col: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) @interpolate(flat) tex_mix: f32,
    @location(3) @interpolate(flat) layer: f32,
    @location(4) @interpolate(flat) layer2: f32,
    @location(5) @interpolate(flat) bmix: f32,
    @location(6) @interpolate(flat) vid: f32,
    @location(7) @interpolate(flat) vido: f32,
}

fn hash11(n: f32) -> f32 {
    return fract(sin(n * 12.9898) * 43758.5453);
}

fn quad_off(corner: u32) -> vec2<f32> {
    var off = vec2(-0.5, -0.5);
    if (corner == 1u || corner == 3u) {
        off = vec2(0.5, -0.5);
    }
    if (corner == 2u || corner == 5u) {
        off = vec2(-0.5, 0.5);
    }
    if (corner == 4u) {
        off = vec2(0.5, 0.5);
    }
    return off;
}

fn emit(
    p: vec2<f32>,
    sz: vec2<f32>,
    corner: u32,
    col: vec3<f32>,
    a: f32,
    uvc: vec2<f32>,
    tex_mix: f32,
    layer: f32,
    layer2: f32,
    bmix: f32,
    vid: f32,
    vido: f32,
) -> VsOut {
    var out: VsOut;
    let off = quad_off(corner);
    let q = p + off * sz;
    let ndc = vec2(q.x / u.resolution.x * 2.0 - 1.0, 1.0 - q.y / u.resolution.y * 2.0);
    out.pos = vec4(ndc, 0.0, 1.0);
    out.col = vec4(col, clamp(a, 0.0, 1.0) * clamp(u.vis, 0.0, 1.0));
    out.uv = uvc + off * (vec2(1.15, 1.15) / u.grid);
    out.tex_mix = tex_mix;
    out.layer = layer;
    out.layer2 = layer2;
    out.bmix = bmix;
    out.vid = vid;
    out.vido = vido;
    return out;
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    let pid = vi / 6u;
    let corner = vi % 6u;
    let gwu = u32(u.grid.x + 0.5);
    let ng = gwu * u32(u.grid.y + 0.5);
    let role_b = pid >= ng;
    let g = pid % max(ng, 1u);
    let gx = g % max(gwu, 1u);
    let gy = g / max(gwu, 1u);
    let uvp = (vec2(f32(gx), f32(gy)) + vec2(0.5, 0.5)) / u.grid;
    let dc = uvp - vec2(0.5);
    let seed = f32(g) * 0.618034 + select(0.0, 31.7, role_b);
    let h = hash11(seed);
    let h2 = hash11(seed + 47.0);
    let h3 = hash11(seed + 91.0);
    let home = u.center + dc * 2.0 * u.hero;
    let cell = 2.0 * u.hero / u.grid;

    let style = u32(clamp(u.swap_style, 0.0, 17.0) + 0.5);
    let ang01 = fract(atan2(dc.y, dc.x) * 0.15915494 + 1.0 + (h - 0.5) * 0.02);
    let strand_lin = clamp(uvp.y + (h - 0.5) * 0.07, 0.0, 0.999);
    var strand_base = strand_lin;
    if (style == 1u || style == 13u || style == 17u) {
        strand_base = ang01;
    }
        let strand = floor(strand_base * max(u.strands, 2.0));
    let odd = (u32(strand) % 2u) == 1u;
    let pos_front = select(1.0 - uvp.x, uvp.x, u.dir > 0.0);
    let sh = hash11(strand * 7.31 + u.seed * 53.0);
    let sh2 = hash11(strand * 3.77 + u.seed * 29.0);
    let nd = mix(
        clamp(max(abs(dc.x), abs(dc.y)) * 2.0, 0.0, 1.0),
        clamp(length(dc) * 1.4142136, 0.0, 1.0),
        0.25,
    );
    let rim = pow(nd, 1.5);
    let sink = u.center + (vec2(h2, h3) - 0.5) * u.hero * 0.12;
    let ring_axes = vec2(u.hero.x * 0.72, u.hero.y * 0.82) * clamp(u.ring_size, 0.25, 3.0);
    let ra17 = atan2(dc.y, dc.x);
    let ang17 = ra17 + u.time * 1.8 * u.ring_spin * u.dir;
    let wob17 = (sin(ra17 * 3.0 + u.time * 1.7) * 0.07
        + sin(ra17 * 5.0 - u.time * 2.3 + hash11(f32(g) * 0.618034 + 12.9) * 6.2831853) * 0.05)
        * u.ring_wave;
    let rr17 = 1.0 + wob17 + (hash11(f32(g) * 0.618034 + 12.9) - 0.5) * 0.12;
    let ring_a = u.center + vec2(cos(ang17), sin(ang17)) * ring_axes * rr17;
    let ring_b = u.center
        + vec2(
            cos(ang17 - 0.9 * u.dir),
            sin(ang17 - 0.9 * u.dir),
        ) * ring_axes * rr17;
    let start17 = (rim * 0.80 + h * 0.20) * 0.60;
    let raw17 = (u.progress - start17) / 0.40;
    let local17 = clamp(raw17, 0.0, 1.0);
    let gather17 = 0.30;
    let scatter17 = 0.62;
    let entry17 = ra17 + (h - 0.5) * 0.04;
    let spin17 = 6.2831853 * (0.55 + sh2 * 0.35) * u.dir;
    let axes17 = ring_axes * rr17;
    var ring_curve17 = home;
    var ramt17 = 0.0;
    var cell17 = 1.0;
    if (raw17 > 0.0 && raw17 < 1.0) {
        if (local17 < gather17) {
            let t17 = local17 / gather17;
            let e17 = t17 * t17 * (3.0 - 2.0 * t17);
            let ringp17 =
                u.center + vec2(cos(entry17), sin(entry17)) * axes17;
            let tangent17 =
                normalize(vec2(-sin(entry17) * axes17.x, cos(entry17) * axes17.y));
            let waypoint17 =
                mix(home, ringp17, 0.55) + tangent17 * u.hero.y * 0.14 * u.dir;
            ring_curve17 =
                mix(mix(home, waypoint17, e17), mix(waypoint17, ringp17, e17), e17);
            ramt17 = sin(1.5707963 * e17);
            cell17 = 1.0 - smoothstep(0.03, 0.68, e17);
        } else if (local17 < scatter17) {
            let t17 = (local17 - gather17) / (scatter17 - gather17);
            let a17 = entry17 + spin17 * t17;
            let orbit_wobble17 =
                1.0 + sin(t17 * 6.2831853 * (1.0 + sh * 2.0) + h * 6.2831853) * 0.05;
            ring_curve17 =
                u.center + vec2(cos(a17), sin(a17)) * axes17 * orbit_wobble17;
            ramt17 = 1.0;
            cell17 = 0.0;
        } else {
            let t17 = (local17 - scatter17) / (1.0 - scatter17);
            let e17 = t17 * t17 * (3.0 - 2.0 * t17);
            let exit_ang17 = entry17 + spin17;
            let ringp17 =
                u.center + vec2(cos(exit_ang17), sin(exit_ang17)) * axes17;
            let tangent17 =
                normalize(vec2(-sin(exit_ang17) * axes17.x, cos(exit_ang17) * axes17.y));
            let waypoint17 =
                mix(ringp17, home, 0.45) + tangent17 * u.hero.y * 0.11 * u.dir;
            ring_curve17 =
                mix(mix(ringp17, waypoint17, e17), mix(waypoint17, home, e17), e17);
            ramt17 = sin(1.5707963 * (1.0 - e17));
            cell17 = smoothstep(0.70, 0.98, e17);
        }
        let activity17 = sin(3.14159265 * local17);
        ring_curve17 += vec2(
            sin(local17 * 9.0 + h * 6.2831853),
            cos(local17 * 7.0 + h2 * 6.2831853),
        ) * u.hero.y * 0.025 * activity17;
    }
    let dust17 = 1.0 - cell17;
    let grain17 = 1.15 + ramt17 * 1.25;
    let exit_x = select(
        u.resolution.x + u.hero.x * 0.4,
        -u.hero.x * 0.4,
        u.dir > 0.0,
    );
    let entry_x = select(
        -u.hero.x * 0.4,
        u.resolution.x + u.hero.x * 0.4,
        u.dir > 0.0,
    );
    let neck = vec2(u.center.x + (h2 - 0.5) * u.hero.x * 0.06, u.center.y + u.hero.y);
    let hx = home.x + (h2 - 0.5) * u.hero.x * 0.25;
    let xn = (hx - u.center.x) / max(u.hero.x, 1.0);
    let heap = vec2(hx, u.center.y + u.hero.y * (1.0 - exp(-xn * xn * 2.8) * (0.15 + h3 * 0.55)));
    let ground = u.center.y + u.hero.y * 0.92;
    let oa0 = h * 6.2831853;
    let osw = (2.0 + sh2 * 1.6) * u.dir;
    let ring_ax = vec2(u.hero.x, u.hero.y * 0.88) * (1.15 + h3 * 0.40);
    let orb_a1 = u.center + vec2(cos(oa0 + osw * 0.5), sin(oa0 + osw * 0.5)) * ring_ax;
    let orb_a2 = u.center + vec2(cos(oa0 + osw), sin(oa0 + osw)) * ring_ax;
    let ob0 = h2 * 6.2831853;
    let obw = ob0 + (1.6 + sh * 1.4) * u.dir;
    let ring_bx = vec2(u.hero.x, u.hero.y * 0.88) * (1.15 + h * 0.40);
    let orb_b0 = u.center + vec2(cos(ob0), sin(ob0)) * ring_bx;
    let orb_b1 = u.center + vec2(cos(obw), sin(obw)) * ring_bx;
    let rv = dc + (vec2(h, h2) - 0.5) * 0.02;
    let dcn = rv / max(length(rv), 1e-4);
    let far = u.center + dcn * length(u.resolution) * 0.62;
    let pa = (strand + 0.5) / max(u.strands, 2.0) * 6.2831853;
    let bloom_w = u.center
        + vec2(cos(pa + 1.1 * u.dir), sin(pa + 1.1 * u.dir) * 0.85) * u.hero.y * 1.3;
    let bloom_out = u.center
        + vec2(cos(pa + 2.2 * u.dir), sin(pa + 2.2 * u.dir) * 0.85) * length(u.resolution) * 0.55;
    let bloom_bw = u.center
        + vec2(cos(pa - 1.1 * u.dir), sin(pa - 1.1 * u.dir) * 0.85) * u.hero.y * 1.3;
    let bloom_b0 = u.center
        + vec2(cos(pa - 2.2 * u.dir), sin(pa - 2.2 * u.dir) * 0.85) * length(u.resolution) * 0.55;
    let q_s = clamp(pos_front * 0.8 + sh * 0.2, 0.0, 0.99);
    let serp_amp = u.hero.y * 0.7 * u.waist;

    var col: vec3<f32>;
    var tt: f32;
    var p0: vec2<f32>;
    var p2: vec2<f32>;
    var fade: f32;
    var szf: vec2<f32>;
    var layer = u.layer_a;
    var layer2 = u.layer_a;
    var bm = 1.0;
    var extra_mid = 0.0;
    var ring_pos = vec2(0.0, 0.0);
    var swg = 0.0;
    var vid = 0.0;
    var vido = 0.0;

    let sh3 = hash11(strand * 9.13 + u.seed * 71.0);
    let hc = hash11(f32(g) * 0.618034 + 12.9);
    let bmixc = clamp(u.bmix, 0.0, 1.0);
    let loop_style = u.swap_loop > 0.5;
    let wk = clamp(pos_front * 0.75 + hc * 0.25, 0.0, 0.999);
    let arch_t = clamp((bmixc * 1.111 - wk * 0.58) / 0.42, 0.0, 1.0);
    let has_wave = u.wave_flag > 0.5;
    let cswitch = smoothstep(0.4, 0.6, arch_t);
    let cring = smoothstep(hc * 0.7, hc * 0.7 + 0.3, bmixc);
    var cmix = select(cring, cswitch, has_wave);
    if (style == 17u && u.swirl < 0.001) {
        let w17 = clamp((u.progress - 0.30) / 0.50, 0.0, 1.0);
        cmix = smoothstep(hc * 0.55, hc * 0.55 + 0.45, w17);
    }
    let solid_kill = select(1.0, 0.0, u.progress >= 0.999);

    if (!role_b) {
        col = textureSampleLevel(near_tex, samp, uvp, i32(u.layer_a), 0.0).rgb;
        let lum = dot(col, vec3(0.299, 0.587, 0.114));
        if (u.video_out > 0.001) {
            let vcol = textureSampleLevel(prev_out_tex, samp, uvp, 0.0).rgb;
            col = mix(col, vcol, u.video_out);
        }
        vido = u.video_out;
        let ma = (1.35 + u.front) / 0.90;
        var dep = pos_front;
        var fa = vec2(0.55, 0.95);
        p0 = home;
        p2 = vec2(exit_x, u.center.y + (sh - 0.5) * u.resolution.y * u.fan);
        if (style == 1u) {
            dep = rim;
            p2 = sink;
            fa = vec2(0.70, 0.98);
        } else if (style == 2u) {
            dep = uvp.y;
            p2 = neck;
            fa = vec2(0.72, 0.985);
        } else if (style == 3u) {
            dep = uvp.y;
            p2 = heap;
            fa = vec2(0.82, 0.98);
        } else if (style == 6u) {
            p2 = vec2(exit_x, ground + (sh - 0.5) * u.hero.y * 0.12);
        } else if (style == 7u) {
            dep = clamp(length((uvp - vec2(0.5, 1.0)) * vec2(1.0, 0.8)) * 0.95, 0.0, 1.0);
            p2 = vec2(u.center.x + (h2 - 0.5) * u.hero.x * 0.12, u.resolution.y + u.hero.y * 0.6);
            fa = vec2(0.75, 0.98);
        } else if (style == 8u) {
            dep = rim;
            p2 = orb_a2;
            fa = vec2(0.70, 0.97);
        } else if (style == 10u) {
            dep = rim;
            p2 = far;
            fa = vec2(0.55, 0.92);
        } else if (style == 11u) {
            dep = select(pos_front, 1.0 - pos_front, odd);
            p2 = vec2(select(exit_x, entry_x, odd), home.y + (sh - 0.5) * u.hero.y * 0.3);
        } else if (style == 13u) {
            dep = 1.0 - rim;
            p2 = bloom_out;
            fa = vec2(0.60, 0.95);
        } else if (style == 16u) {
            p2 = vec2(exit_x, u.center.y + (sh - 0.5) * u.resolution.y * 0.9);
        } else if (style == 17u) {
            dep = h3;
            p2 = ring_a;
            fa = vec2(0.75, 0.98);
        }
        var pw = 1.0;
        if (style == 2u || style == 3u || style == 7u || style == 10u || style == 13u
            || style == 17u)
        {
            pw = 0.55;
        }
        tt = clamp(
            u.progress / pw * ma - dep * u.front - h * 0.2 - lum * 0.15 + u.carry,
            0.0,
            1.0,
        );
        let ease0 = tt * tt * (3.0 - 2.0 * tt);
        fade = (1.0 - smoothstep(fa.x, fa.y, tt)) * solid_kill;
        fade = fade * (1.0 - smoothstep(0.05, 0.55, u.swirl));
        szf = mix(cell * 1.15, vec2(2.0, 2.0), vec2(ease0, ease0));
        col = col * (1.0 + ease0 * 0.4);
        if (style == 17u) {
            fade = 0.0;
            szf = vec2(0.0, 0.0);
        }
    } else {
        let cb_new = textureSampleLevel(near_tex, samp, uvp, i32(u.layer_b), 0.0).rgb;
        let cb_old = textureSampleLevel(near_tex, samp, uvp, i32(u.layer_b2), 0.0).rgb;
        var cb_base = cb_old;
        if (u.bcut < 0.999) {
            let cb_o3 = textureSampleLevel(near_tex, samp, uvp, i32(u.layer_b3), 0.0).rgb;
            cb_base = mix(cb_o3, cb_old, smoothstep(hc * 0.7, hc * 0.7 + 0.3, u.bcut));
        }
        col = mix(cb_base, cb_new, cmix);
        if (u.video_in > 0.001) {
            let vcol = textureSampleLevel(prev_tex, samp, uvp, 0.0).rgb;
            col = mix(col, vcol, u.video_in * cmix);
        }
        let lum = dot(cb_old, vec3(0.299, 0.587, 0.114));
        let mb = (1.32 + u.front) / 0.86;
        var dep = pos_front;
        var ab = vec2(0.02, 0.18);
        p0 = vec2(entry_x, u.center.y + (sh - 0.5) * u.resolution.y * u.fan);
        p2 = home;
        if (style == 1u) {
            dep = rim;
            p0 = sink;
        } else if (style == 2u) {
            dep = 1.0 - uvp.y;
            p0 = neck + vec2(0.0, 4.0);
        } else if (style == 3u) {
            dep = 1.0 - uvp.y;
            p0 = heap;
            ab = vec2(0.03, 0.15);
        } else if (style == 6u) {
            p0 = vec2(entry_x, ground + (sh2 - 0.5) * u.hero.y * 0.12);
        } else if (style == 7u) {
            dep = 1.0 - uvp.y;
            p0 = vec2(u.center.x + (h3 - 0.5) * u.hero.x * 0.12, -u.hero.y * 0.6);
            ab = vec2(0.03, 0.16);
        } else if (style == 8u) {
            dep = rim;
            p0 = orb_b0;
            ab = vec2(0.05, 0.25);
        } else if (style == 10u) {
            dep = rim;
            p0 = far;
            ab = vec2(0.03, 0.18);
        } else if (style == 11u) {
            dep = select(pos_front, 1.0 - pos_front, odd);
            p0 = vec2(select(entry_x, exit_x, odd), home.y + (sh2 - 0.5) * u.hero.y * 0.3);
        } else if (style == 13u) {
            dep = rim;
            p0 = bloom_b0;
            ab = vec2(0.03, 0.20);
        } else if (style == 16u) {
            p0 = vec2(entry_x, u.center.y + (sh2 - 0.5) * u.resolution.y * 0.9);
        } else if (style == 17u) {
            dep = h2;
            p0 = ring_b;
        }
        var pb = u.progress;
        if (style == 2u || style == 3u || style == 7u || style == 10u || style == 13u
            || style == 17u)
        {
            pb = (u.progress - 0.35) / 0.55;
        }
        tt = clamp(
            (pb - 0.04) * mb - dep * u.front - h * 0.2 - (1.0 - lum) * 0.12,
            0.0,
            1.0,
        );
        if (style == 17u) {
            tt = 1.0 - ramt17;
        }
        let ease0 = tt * tt * (3.0 - 2.0 * tt);
        fade = smoothstep(ab.x, ab.y, tt) * solid_kill;
        szf = mix(vec2(2.0, 2.0), cell * 1.15, vec2(ease0, ease0));
        col = col * (1.0 + (1.0 - ease0) * 0.4);
        if (style == 17u) {
            fade = solid_kill;
            szf = mix(vec2(grain17, grain17), cell * 1.15, vec2(cell17, cell17));
            extra_mid = max(extra_mid, dust17);
        }
        layer = u.layer_b;
        layer2 = u.layer_b2;
        bm = cmix;
        vid = u.video_in * cmix;
        if (u.swirl > 0.001) {
            swg = smoothstep(hc * 0.35, hc * 0.35 + 0.65, u.swirl);
            let ba = atan2(dc.y, dc.x);
            let ang = ba + u.time * 1.8 * u.ring_spin * u.dir;
            let wob = (sin(ba * 3.0 + u.time * 1.7) * 0.07
                + sin(ba * 5.0 - u.time * 2.3 + hc * 6.2831853) * 0.05)
                * u.ring_wave;
            let rr = 1.0 + wob + (hc - 0.5) * 0.12;
            ring_pos = u.center + vec2(cos(ang), sin(ang)) * ring_axes * rr;
            fade = max(fade, swg * solid_kill / max(u.ring_soft, 0.34));
            extra_mid = max(extra_mid, swg);
        }
    }

    let ease = tt * tt * (3.0 - 2.0 * tt);
    let mid = sin(3.14159265 * ease);

    var waist: vec2<f32>;
    if (style == 1u) {
        let rel = home - u.center;
        let rl = length(rel);
        let ra = atan2(rel.y, rel.x);
        let spin = (0.5 + sh2 * 0.9) * u.dir * (0.4 + u.fan);
        waist = u.center
            + vec2(cos(ra + spin), sin(ra + spin)) * rl * (0.22 + sh * 0.22)
                * (0.45 + u.waist * 0.4)
            + (vec2(h2, h3) - 0.5) * u.hero.y * 0.05;
    } else if (style == 2u) {
        let side = sign(dc.x + (h - 0.5) * 0.2);
        if (role_b) {
            waist = u.center
                + vec2(side * u.hero.x * (0.7 + sh * 0.5), u.hero.y * (0.45 + sh2 * 0.35));
        } else {
            waist = u.center
                + vec2(side * u.hero.x * (0.9 + sh * 0.5), (sh2 - 0.6) * u.hero.y * 0.5);
        }
        waist += (vec2(h2, h3) - 0.5) * u.hero.y * 0.12 * u.waist;
    } else if (style == 3u) {
        let side = sign(dc.x + (h - 0.5) * 0.2);
        if (role_b) {
            waist = vec2(
                mix(heap.x, home.x, 0.3) + side * u.hero.x * (0.15 + sh * 0.25),
                u.center.y - u.hero.y * (0.75 + sh2 * 0.5),
            );
        } else {
            waist = vec2(
                home.x + side * u.hero.x * (0.5 + sh * 0.7),
                home.y - u.hero.y * (0.45 + sh2 * 0.55),
            );
        }
    } else if (style == 6u) {
        waist = vec2(
            mix(p0.x, p2.x, 0.35 + sh2 * 0.3),
            ground - u.hero.y * (0.05 + sh * 0.08) * u.waist,
        );
    } else if (style == 7u) {
        if (role_b) {
            waist = vec2(u.center.x + (sh2 - 0.5) * u.hero.x * 0.15, u.resolution.y * 0.08);
        } else {
            waist = vec2(u.center.x + (sh - 0.5) * u.hero.x * 0.15, u.resolution.y * 0.94);
        }
    } else if (style == 8u) {
        if (role_b) {
            waist = orb_b1;
        } else {
            waist = orb_a1;
        }
        waist += (vec2(h2, h3) - 0.5) * u.hero.y * 0.18 * u.waist;
    } else if (style == 10u) {
        let tang = vec2(-dcn.y, dcn.x) * (sh - 0.5) * u.hero.y * 0.9 * u.dir;
        waist = mix(p0, p2, 0.5) + tang;
    } else if (style == 11u) {
        waist = vec2(
            mix(p0.x, p2.x, 0.3 + sh2 * 0.4),
            home.y + (h2 - 0.5) * u.hero.y * 0.3 * u.waist,
        );
    } else if (style == 13u) {
        if (role_b) {
            waist = bloom_bw;
        } else {
            waist = bloom_w;
        }
        waist += (vec2(h2, h3) - 0.5) * u.hero.y * 0.10 * u.waist;
    } else if (style == 16u) {
        waist = vec2(
            mix(p0.x, p2.x, 0.25 + sh2 * 0.5),
            u.center.y + (sh - 0.5) * u.resolution.y * 0.45,
        );
    } else if (style == 17u) {
        if (role_b) {
            waist = mix(p0, p2, 0.45) + (vec2(h2, h3) - 0.5) * u.hero.y * 0.25 * u.waist;
        } else {
            waist = mix(p0, p2, 0.55) + (vec2(h2, h3) - 0.5) * u.hero.y * 0.25 * u.waist;
        }
    } else {
        waist = vec2(
            mix(p0.x, p2.x, 0.3 + sh2 * 0.4),
            u.center.y
                + ((sh - 0.5) * u.hero.y * 0.3 + (h2 - 0.5) * u.hero.y * 0.05) * u.waist,
        );
    }

    let q0 = mix(p0, waist, ease);
    let q1 = mix(waist, p2, ease);
    var guide = mix(q0, q1, ease);
    if (style == 17u && role_b) {
        guide = ring_curve17;
    }

    var turb_mul = 1.0;
    var orb_mul = 1.0;
    if (style == 2u) {
        turb_mul = 1.4;
        orb_mul = 1.6;
    }
    if (style == 3u) {
        turb_mul = 1.3;
        orb_mul = 1.5;
    }
    if (style == 6u) {
        turb_mul = 0.4;
        orb_mul = 0.3;
    }
    if (style == 7u) {
        turb_mul = 1.2;
        orb_mul = 1.2;
    }
    if (style == 8u) {
        turb_mul = 1.2;
        orb_mul = 1.4;
    }
    if (style == 10u) {
        turb_mul = 1.2;
        orb_mul = 1.3;
    }
    if (style == 11u) {
        turb_mul = 1.25;
        orb_mul = 1.25;
    }
    if (style == 13u) {
        turb_mul = 1.2;
        orb_mul = 1.4;
    }
    if (style == 16u) {
        turb_mul = 1.6;
        orb_mul = 0.5;
    }
    if (style == 17u) {
        turb_mul = 0.7;
        orb_mul = 0.6;
    }
    guide += vec2(
        sin(ease * (4.0 + sh3 * 5.0) * 3.14159265 + h * 6.2831853 + u.time * 0.5),
        cos(ease * (3.0 + sh2 * 4.0) * 3.14159265 + h2 * 6.2831853 + u.time * 0.42),
    ) * u.hero.y * (0.05 + h3 * 0.07) * mid * u.turbulence * turb_mul;

    let orb_r = (6.0 + h2 * 22.0) * u.orbit * orb_mul * mid;
    let orb_ang = ease * 6.2831853 * (1.5 + sh3 * 1.8) * u.twist
        + uvp.x * (7.0 + sh3 * 7.0) * u.twist
        + h2 * 1.3
        + u.time * 0.6;

    var arch_off = vec2(0.0, 0.0);
    if (has_wave && loop_style && role_b) {
        let damp = 1.0 - mid * 0.6;
        let arch_r = u.hero.y * (0.22 + h2 * 0.22 + h3 * 0.10) * u.arc * damp;
        let theta = arch_t * 6.2831853;
        let cu = normalize(vec2(u.dir * 0.85, -1.0));
        let cw = vec2(cu.y, -cu.x) * u.dir;
        arch_off = (cw * sin(theta) + cu * (1.0 - cos(theta))) * arch_r;
    }
    var style_off = vec2(0.0, 0.0);
    if (style == 6u) {
        style_off.y = -abs(sin(ease * 3.14159265 * (3.0 + sh3 * 3.0) + h * 3.0))
            * u.hero.y * (0.10 + h3 * 0.08) * mid;
    }
    if (style == 16u) {
        let lr = u.hero.y * (0.25 + sh2 * 0.25) * mid;
        let la = ease * 6.2831853 * (1.0 + sh3 * 1.2) * u.dir + h * 6.2831853;
        style_off = vec2(cos(la), sin(la)) * lr;
    }
    var p = guide + vec2(cos(orb_ang), sin(orb_ang)) * orb_r + arch_off + style_off;
    if (swg > 0.001) {
        let ring_ang = atan2(ring_pos.y - u.center.y, ring_pos.x - u.center.x);
        let ring_tangent = normalize(vec2(
            -sin(ring_ang) * ring_axes.x,
            cos(ring_ang) * ring_axes.y,
        ));
        let ring_waypoint =
            mix(p, ring_pos, 0.55)
                + ring_tangent * u.hero.y * 0.13 * clamp(u.ring_size, 0.25, 3.0) * u.dir;
        p = mix(mix(p, ring_waypoint, swg), mix(ring_waypoint, ring_pos, swg), swg);
        let depth_ring = sin(ring_ang) * 0.5 + 0.5;
        let gs = 2.2
            * (1.0 + (max(u.ring_soft, 1.0) - 1.0) * 0.9)
            * mix(0.82, 1.18, depth_ring);
        szf = mix(szf, vec2(gs, gs), vec2(swg, swg));
        col = col * (1.0 + swg * mix(0.12, 0.42, depth_ring));
    }

    let corner_mid = select(0.0, sin(3.14159265 * arch_t), has_wave && loop_style && role_b);
    szf = mix(szf, vec2(2.4, 2.4), vec2(corner_mid * 0.8));
    col = col * (1.0 + corner_mid * 0.35);
    let tex_mix = 1.0 - smoothstep(0.03, 0.3, max(mid, max(corner_mid * 0.9, extra_mid)));
    return emit(p, szf, corner, col, fade, uvp, tex_mix, layer, layer2, bm, vid, vido);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    var rgb = in.col.rgb;
    if (in.tex_mix > 0.003) {
        var t = textureSampleLevel(near_tex, samp, in.uv, i32(in.layer), 0.0).rgb;
        if (in.bmix < 0.997) {
            let t2 = textureSampleLevel(near_tex, samp, in.uv, i32(in.layer2), 0.0).rgb;
            t = mix(t2, t, in.bmix);
        }
        if (in.vid > 0.003) {
            let vt = textureSampleLevel(
                prev_tex,
                samp,
                clamp(in.uv, vec2(0.0, 0.0), vec2(1.0, 1.0)),
                0.0,
            ).rgb;
            t = mix(t, vt, in.vid);
        }
        if (in.vido > 0.003) {
            let vo = textureSampleLevel(
                prev_out_tex,
                samp,
                clamp(in.uv, vec2(0.0, 0.0), vec2(1.0, 1.0)),
                0.0,
            ).rgb;
            t = mix(t, vo, in.vido);
        }
        rgb = mix(rgb, t, in.tex_mix);
    }
    return vec4(rgb * in.col.a, in.col.a);
}
