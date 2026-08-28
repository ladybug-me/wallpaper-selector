use super::EffectValues;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct ShaderSpec {
    pub effect: u32,
    pub params: [f32; 4],
    pub color_a: [f32; 4],
    pub color_b: [f32; 4],
}

#[must_use]
pub fn shader_spec(effect_id: &str, values: &EffectValues) -> Option<ShaderSpec> {
    let number = |key: &str, default: f64| {
        values.get(key).and_then(super::EffectValue::as_f64).unwrap_or(default)
    };
    let text = |key: &str, default: &str| {
        values.get(key).and_then(super::EffectValue::as_str).unwrap_or(default).to_string()
    };
    let mut spec = ShaderSpec::default();
    match effect_id {
        "brightness" => {
            spec.effect = 1;
            spec.params[0] = number("factor", 1.1) as f32;
        }
        "contrast" => {
            if text("mode", "normal") == "sigmoid" {
                return None;
            }
            spec.effect = 2;
            spec.params[0] = ((number("factor", 25.0) + 100.0) / 100.0) as f32;
        }
        "saturation" => {
            spec.effect = 3;
            spec.params[0] = (1.0 + number("percentage", 25.0) / 100.0) as f32;
        }
        "gamma" => {
            spec.effect = 4;
            spec.params[0] = (1.0 / number("gamma", 1.0).max(0.001)) as f32;
        }
        "invert" => spec.effect = 5,
        "grayscale" => spec.effect = 6,
        "sepia" => {
            spec.effect = 7;
            spec.params[0] = number("intensity", 1.0).clamp(0.0, 1.0) as f32;
        }
        "hue" => {
            spec.effect = 8;
            spec.params[0] = (number("degrees", 180.0) / 360.0) as f32;
        }
        "temperature" => {
            spec.effect = 9;
            let red_delta = (number("amount", 30.0) / 100.0 * 50.0) as f32;
            spec.params[0] = red_delta / 255.0;
            spec.params[1] = -red_delta / 255.0;
        }
        "tint" => {
            spec.effect = 10;
            spec.params[0] = number("opacity", 0.35).clamp(0.0, 1.0) as f32;
            spec.color_a = color(values, "color", "#7aa2f7");
        }
        "posterize" => {
            spec.effect = 11;
            spec.params[0] = number("levels", 6.0).clamp(2.0, 256.0) as f32;
        }
        "solarize" => {
            spec.effect = 12;
            spec.params[0] = number("threshold", 128.0).clamp(0.0, 255.0) as f32 / 255.0;
        }
        "threshold" => {
            spec.effect = 13;
            spec.params[0] = number("cutoff", 128.0).clamp(0.0, 255.0) as f32 / 255.0;
        }
        "duotone" => {
            spec.effect = 14;
            spec.color_a = color(values, "shadow", "#1e1e2e");
            spec.color_b = color(values, "highlight", "#cdd6f4");
        }
        "vignette" => {
            spec.effect = 15;
            spec.params[0] = number("strength", 0.6).clamp(0.0, 1.0) as f32;
            spec.params[1] = number("radius", 0.7).clamp(0.05, 2.0) as f32;
        }
        "dim" => {
            spec.effect = 16;
            spec.params[0] = number("strength", 0.5).clamp(0.0, 1.0) as f32;
            spec.params[1] = (number("size", 25.0).clamp(1.0, 100.0) / 100.0) as f32;
            spec.params[2] = match text("edge", "top").as_str() {
                "bottom" => 1.0,
                "both" => 2.0,
                _ => 0.0,
            };
        }
        "flip" => spec.effect = 17,
        "mirror" => spec.effect = 18,
        _ => return None,
    }
    Some(spec)
}

fn color(values: &EffectValues, key: &str, default: &str) -> [f32; 4] {
    let text = values.get(key).and_then(super::EffectValue::as_str).unwrap_or(default);
    rgba(text)
}

fn rgba(text: &str) -> [f32; 4] {
    let hex = text.trim().trim_start_matches('#').as_bytes();
    let rgb: &[u8] = match hex.len() {
        6 => &hex[0..6],
        8 => &hex[2..8],
        _ => b"000000",
    };
    let channel = |index: usize| {
        let pair = std::str::from_utf8(&rgb[index..index + 2]).unwrap_or("zz");
        f32::from(u8::from_str_radix(pair, 16).unwrap_or(0)) / 255.0
    };
    [channel(0), channel(2), channel(4), 1.0]
}

mod tests;
