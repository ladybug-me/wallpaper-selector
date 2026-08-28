struct Uniforms {
    bounds: vec4<f32>,
    tex_size: vec2<f32>,
    effect: u32,
    out_srgb: u32,
    params: vec4<f32>,
    color_a: vec4<f32>,
    color_b: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var src_tex: texture_2d<f32>;
@group(0) @binding(2) var src_samp: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VsOut {
    var out: VsOut;
    let uv = vec2<f32>(f32((idx << 1u) & 2u), f32(idx & 2u));
    out.pos = vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
    return out;
}

fn luma(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.299, 0.587, 0.114));
}

fn srgb_to_linear(c: vec3<f32>) -> vec3<f32> {
    let lo = c / 12.92;
    let hi = pow((c + 0.055) / 1.055, vec3<f32>(2.4));
    return select(hi, lo, c <= vec3<f32>(0.04045));
}

fn rgb2hsl(c: vec3<f32>) -> vec3<f32> {
    let mx = max(c.r, max(c.g, c.b));
    let mn = min(c.r, min(c.g, c.b));
    let l = (mx + mn) * 0.5;
    let d = mx - mn;
    if (d < 1e-6) {
        return vec3<f32>(0.0, 0.0, l);
    }
    var s: f32;
    if (l > 0.5) {
        s = d / (2.0 - mx - mn);
    } else {
        s = d / (mx + mn);
    }
    var h: f32;
    if (abs(mx - c.r) < 1e-6) {
        h = (c.g - c.b) / d + select(0.0, 6.0, c.g < c.b);
    } else if (abs(mx - c.g) < 1e-6) {
        h = (c.b - c.r) / d + 2.0;
    } else {
        h = (c.r - c.g) / d + 4.0;
    }
    return vec3<f32>(h / 6.0, s, l);
}

fn hue_channel(p: f32, q: f32, t0: f32) -> f32 {
    var t = t0;
    if (t < 0.0) {
        t += 1.0;
    }
    if (t > 1.0) {
        t -= 1.0;
    }
    if (t < 1.0 / 6.0) {
        return p + (q - p) * 6.0 * t;
    }
    if (t < 0.5) {
        return q;
    }
    if (t < 2.0 / 3.0) {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    return p;
}

fn hsl2rgb(h: f32, s: f32, l: f32) -> vec3<f32> {
    if (abs(s) < 1e-6) {
        return vec3<f32>(l);
    }
    var q: f32;
    if (l < 0.5) {
        q = l * (1.0 + s);
    } else {
        q = l + s - l * s;
    }
    let p = 2.0 * l - q;
    return vec3<f32>(
        hue_channel(p, q, h + 1.0 / 3.0),
        hue_channel(p, q, h),
        hue_channel(p, q, h - 1.0 / 3.0),
    );
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let uv0 = (in.pos.xy - u.bounds.xy) / u.bounds.zw;
    let ba = u.bounds.z / u.bounds.w;
    let ta = u.tex_size.x / u.tex_size.y;
    var uv = uv0;
    if (ta > ba) {
        let s = ba / ta;
        uv.x = (uv0.x - 0.5) * s + 0.5;
    } else {
        let s = ta / ba;
        uv.y = (uv0.y - 0.5) * s + 0.5;
    }

    let e = u.effect;
    if (e == 17u) {
        uv.y = 1.0 - uv.y;
    } else if (e == 18u) {
        uv.x = 1.0 - uv.x;
    }

    var c = textureSample(src_tex, src_samp, uv).rgb;
    let p = u.params;
    if (e == 1u) {
        c = c * p.x;
    } else if (e == 2u) {
        c = (c - vec3<f32>(0.5)) * p.x + vec3<f32>(0.5);
    } else if (e == 3u) {
        let l = luma(c);
        c = vec3<f32>(l) + (c - vec3<f32>(l)) * p.x;
    } else if (e == 4u) {
        c = pow(max(c, vec3<f32>(0.0)), vec3<f32>(p.x));
    } else if (e == 5u) {
        c = vec3<f32>(1.0) - c;
    } else if (e == 6u) {
        c = vec3<f32>(luma(c));
    } else if (e == 7u) {
        let sr = 0.393 * c.r + 0.769 * c.g + 0.189 * c.b;
        let sg = 0.349 * c.r + 0.686 * c.g + 0.168 * c.b;
        let sb = 0.272 * c.r + 0.534 * c.g + 0.131 * c.b;
        c = c + (vec3<f32>(sr, sg, sb) - c) * p.x;
    } else if (e == 8u) {
        let hsl = rgb2hsl(c);
        c = hsl2rgb(fract(hsl.x + p.x), hsl.y, hsl.z);
    } else if (e == 9u) {
        c.r = c.r + p.x;
        c.b = c.b + p.y;
    } else if (e == 10u) {
        c = mix(c, u.color_a.rgb, p.x);
    } else if (e == 11u) {
        let lvl = round(c * (p.x - 1.0));
        c = round(lvl * 255.0 / (p.x - 1.0)) / 255.0;
    } else if (e == 12u) {
        c = select(c, vec3<f32>(1.0) - c, c >= vec3<f32>(p.x));
    } else if (e == 13u) {
        c = vec3<f32>(select(0.0, 1.0, luma(c) >= p.x));
    } else if (e == 14u) {
        c = mix(u.color_a.rgb, u.color_b.rgb, luma(c));
    } else if (e == 15u) {
        let d = (uv - vec2<f32>(0.5)) * u.tex_size;
        let dist = length(d) / length(u.tex_size * 0.5);
        let fade = clamp((dist - p.y) / max(1.0 - p.y, 1e-3), 0.0, 1.0);
        c = c * (1.0 - fade * p.x);
    } else if (e == 16u) {
        let band = max(u.tex_size.y * p.y, 1.0);
        let y = uv.y * u.tex_size.y;
        let top_f = clamp((band - y) / band, 0.0, 1.0);
        let bot_f = clamp((y - (u.tex_size.y - band)) / band, 0.0, 1.0);
        var fade = top_f;
        if (p.z == 1.0) {
            fade = bot_f;
        } else if (p.z == 2.0) {
            fade = max(top_f, bot_f);
        }
        c = c * (1.0 - fade * p.x);
    }
    c = clamp(c, vec3<f32>(0.0), vec3<f32>(1.0));

    var outc = c;
    if (u.out_srgb == 1u) {
        outc = srgb_to_linear(c);
    }
    return vec4<f32>(outc, 1.0);
}
