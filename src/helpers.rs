use crate::vectors::Vec3;

pub fn project(
    vec3: Vec3,
    cols: usize,
    rows: usize,
    scale: f32,
    char_aspect: f32,
) -> Option<(f32, f32, f32)> {
    if vec3.z <= 1e-4 {
        return None;
    }

    let x_ndc = vec3.x / vec3.z;
    let y_ndc = vec3.y / vec3.z;

    let px = cols as f32 / 2.0 + x_ndc * scale;
    let py = rows as f32 / 2.0 - y_ndc * scale / char_aspect;

    Some((px, py, 1.0 / vec3.z))
}

pub fn rasterize_flat(
    p0: (f32, f32, f32),
    p1: (f32, f32, f32),
    p2: (f32, f32, f32),
    ch: char,
    color: (u8, u8, u8),
    cols: usize,
    rows: usize,
    grid: &mut [char],
    zbuf: &mut [f32],
    color_buf: &mut [(u8, u8, u8)],
) {
    let (x0, y0, z0) = p0;
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;

    let min_x = x0.min(x1).min(x2).floor().max(0.0) as usize;
    let max_x = x0.max(x1).max(x2).ceil().min(cols as f32 - 1.0) as usize;
    let min_y = y0.min(y1).min(y2).floor().max(0.0) as usize;
    let max_y = y0.max(y1).max(y2).ceil().min(rows as f32 - 1.0) as usize;

    let denom = (y1 - y2) * (x0 - x2) + (x2 - x1) * (y0 - y2);
    if denom.abs() < 1e-6 {
        return;
    }

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            let fx = px as f32 + 0.5;
            let fy = py as f32 + 0.5;

            let w0 = ((y1 - y2) * (fx - x2) + (x2 - x1) * (fy - y2)) / denom;
            let w1 = ((y2 - y0) * (fx - x2) + (x0 - x2) * (fy - y2)) / denom;
            let w2 = 1.0 - w0 - w1;

            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let depth = w0 * z0 + w1 * z1 + w2 * z2;
                let i = py * cols + px;
                if depth > zbuf[i] {
                    zbuf[i] = depth;
                    grid[i] = ch;
                    color_buf[i] = color;
                }
            }
        }
    }
}
