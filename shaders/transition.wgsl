struct TransUniform {
    resolution: vec2<f32>,
    origin: vec2<f32>,
    progress: f32,
    kind: u32,
    _pad0: f32,
    _pad1: f32,
    accent: vec4<f32>,
}

@group(0) @binding(0) var<uniform> u: TransUniform;
@group(0) @binding(1) var tex_a: texture_2d<f32>;
@group(0) @binding(2) var tex_b: texture_2d<f32>;
@group(0) @binding(3) var samp: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    var out: VsOut;
    let x = f32(i32(vi) / 2) * 4.0 - 1.0;
    let y = f32(i32(vi) % 2) * 4.0 - 1.0;
    out.pos = vec4(x, y, 0.0, 1.0);
    out.uv = vec2(x * 0.5 + 0.5, 0.5 - y * 0.5);
    return out;
}

fn hash2(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

fn vnoise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let s = f * f * (3.0 - 2.0 * f);
    let a = hash2(i);
    let b = hash2(i + vec2(1.0, 0.0));
    let c = hash2(i + vec2(0.0, 1.0));
    let d = hash2(i + vec2(1.0, 1.0));
    return mix(mix(a, b, s.x), mix(c, d, s.x), s.y);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let p = clamp(u.progress, 0.0, 1.0);
    let t = p * p * (3.0 - 2.0 * p);
    var uv = in.uv;

    if (u.kind == 1u) {
        let aspect = u.resolution.x / max(u.resolution.y, 1.0);
        let pos = vec2(uv.x * aspect, uv.y);
        let org = vec2(u.origin.x * aspect, u.origin.y);
        let d = distance(pos, org);
        let wave = sin(d * 44.0 - p * 18.0);
        let amp = 0.022 * (1.0 - p) * smoothstep(0.0, 0.2, p);
        let dir = normalize(pos - org + vec2(0.0001, 0.0));
        let off = vec2(dir.x / aspect, dir.y) * wave * amp;
        let uv0 = clamp(uv + off, vec2(0.0), vec2(1.0));
        let uv1 = clamp(uv + off * 1.6, vec2(0.0), vec2(1.0));
        let uv2 = clamp(uv + off * 0.4, vec2(0.0), vec2(1.0));
        let a = vec4(
            textureSample(tex_a, samp, uv1).r,
            textureSample(tex_a, samp, uv0).g,
            textureSample(tex_a, samp, uv2).b,
            textureSample(tex_a, samp, uv0).a,
        );
        let b = vec4(
            textureSample(tex_b, samp, uv1).r,
            textureSample(tex_b, samp, uv0).g,
            textureSample(tex_b, samp, uv2).b,
            textureSample(tex_b, samp, uv0).a,
        );
        var col = mix(a, b, t);
        let wf = p * 1.3;
        let ring = exp(-abs(d - wf) * 26.0) * (1.0 - p) * 1.5;
        col = vec4(col.rgb + u.accent.rgb * ring, max(col.a, ring * 0.5));
        return col;
    }

    if (u.kind == 2u) {
        let n = vnoise(uv * 14.0) * 0.7 + vnoise(uv * 47.0) * 0.3;
        let edge = 0.06;
        let cut = p * (1.0 + edge * 2.0) - edge;
        let m = smoothstep(cut - edge, cut + edge, n);
        let a = textureSample(tex_a, samp, uv);
        let b = textureSample(tex_b, samp, uv);
        let burn = smoothstep(cut - edge, cut, n) * smoothstep(cut + edge, cut, n);
        var col = mix(b, a, m);
        col = vec4(col.rgb + u.accent.rgb * burn * 1.6 * col.a, col.a);
        return col;
    }

    if (u.kind == 3u) {
        let row = floor(uv.y * 36.0);
        let jitter = (hash2(vec2(row, floor(p * 14.0))) - 0.5) * 2.0;
        let strength = sin(p * 3.14159);
        let shift = jitter * 0.08 * strength;
        let uv_s = vec2(clamp(uv.x + shift, 0.0, 1.0), uv.y);
        let split = 0.012 * strength;
        let src_mix = step(hash2(vec2(row, 7.0)), p);
        var col: vec4<f32>;
        if (src_mix > 0.5) {
            col = vec4(
                textureSample(tex_b, samp, vec2(clamp(uv_s.x + split, 0.0, 1.0), uv_s.y)).r,
                textureSample(tex_b, samp, uv_s).g,
                textureSample(tex_b, samp, vec2(clamp(uv_s.x - split, 0.0, 1.0), uv_s.y)).b,
                textureSample(tex_b, samp, uv_s).a,
            );
        } else {
            col = vec4(
                textureSample(tex_a, samp, vec2(clamp(uv_s.x + split, 0.0, 1.0), uv_s.y)).r,
                textureSample(tex_a, samp, uv_s).g,
                textureSample(tex_a, samp, vec2(clamp(uv_s.x - split, 0.0, 1.0), uv_s.y)).b,
                textureSample(tex_a, samp, uv_s).a,
            );
        }
        return col;
    }

    let a = textureSample(tex_a, samp, uv);
    let b = textureSample(tex_b, samp, uv);
    return mix(a, b, t);
}
