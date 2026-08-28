use iced::Color;

fn hsl(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = (h * 6.0).rem_euclid(6.0);
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    Color::from_rgb(r + m, g + m, b + m)
}

pub fn swatch_color(index: usize, bright: bool) -> Color {
    if index == 12 {
        if bright { hsl(0.0, 0.0, 0.6) } else { hsl(0.0, 0.0, 0.45) }
    } else if bright {
        hsl(index as f32 / 12.0, 0.75, 0.55)
    } else {
        hsl(index as f32 / 12.0, 0.65, 0.45)
    }
}

pub fn color_bucket_name(bucket: i64) -> &'static str {
    match bucket {
        0 => "red",
        1 => "orange",
        2 => "yellow",
        3 => "lime",
        4 => "green",
        5 => "mint",
        6 => "cyan",
        7 => "azure",
        8 => "blue",
        9 => "purple",
        10 => "magenta",
        11 => "pink",
        _ => "gray",
    }
}

pub fn parse_color_bucket(name: &str) -> Option<i64> {
    if let Ok(num) = name.parse::<i64>() {
        return Some(num);
    }
    Some(match name.trim().to_lowercase().as_str() {
        "red" => 0,
        "orange" => 1,
        "yellow" => 2,
        "lime" | "chartreuse" => 3,
        "green" => 4,
        "spring" | "mint" | "emerald" => 5,
        "cyan" | "teal" | "aqua" => 6,
        "azure" | "sky" => 7,
        "blue" => 8,
        "violet" | "purple" | "indigo" => 9,
        "magenta" | "fuchsia" => 10,
        "pink" | "rose" => 11,
        "gray" | "grey" | "mono" | "monochrome" | "grayscale" | "greyscale" | "black" | "white" => {
            99
        }
        _ => return None,
    })
}

pub fn strip_bucket(index: usize) -> i64 {
    if index == 12 { 99 } else { index as i64 }
}

pub fn cycle_color_left(current: i64) -> i64 {
    match current {
        -1 | 0 => 99,
        99 => 11,
        val => val - 1,
    }
}

pub fn cycle_color_right(current: i64) -> i64 {
    match current {
        -1 | 99 => 0,
        11 => 99,
        val => val + 1,
    }
}

mod tests;
