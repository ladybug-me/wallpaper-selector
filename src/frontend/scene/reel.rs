use super::InstanceRaw;

pub(crate) fn cell_key(x: f32, y: f32) -> (i32, i32) {
    ((x * 0.5).round() as i32, (y * 0.5).round() as i32)
}

pub(crate) fn roll_in_cut(body: &mut InstanceRaw, fraction: f32) {
    if fraction >= 1.0 {
        return;
    }
    let half_width = body.rect[2];
    body.rect[0] += half_width * (1.0 - fraction);
    body.rect[2] = half_width * fraction;
    body.crop[2] *= fraction;
    for radius in &mut body.radii {
        *radius = radius.min(body.rect[2]).min(body.rect[3]);
    }
}

pub(crate) fn roll_out_cut(body: &mut InstanceRaw, fraction: f32) {
    let half_width = body.rect[2];
    body.rect[0] -= half_width * fraction;
    body.rect[2] = half_width * (1.0 - fraction);
    body.crop[0] += body.crop[2] * fraction;
    body.crop[2] *= 1.0 - fraction;
    for radius in &mut body.radii {
        *radius = radius.min(body.rect[2]).min(body.rect[3]);
    }
}

mod tests;
