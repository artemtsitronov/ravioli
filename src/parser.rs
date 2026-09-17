use crate::vectors::Vec3;
use std::fs;
use std::io;

pub fn parse_obj(path: &str) -> io::Result<(Vec<Vec3>, Vec<[usize; 3]>)> {
    let text = fs::read_to_string(path)?;
    let mut verts = Vec::new();
    let mut tris = Vec::new();

    for line in text.lines() {
        let line = line.trim();

        if let Some(rest) = line.strip_prefix("v ") {
            let nums: Vec<f32> = rest
                .split_whitespace()
                .filter_map(|t| t.parse::<f32>().ok())
                .collect();
            if nums.len() >= 3 {
                verts.push(Vec3::new(nums[0], nums[1], nums[2]));
            }
        } else if let Some(rest) = line.strip_prefix("f ") {
            let idx: Vec<usize> = rest
                .split_whitespace()
                .filter_map(|tok| tok.split('/').next())
                .filter_map(|s| s.parse::<i64>().ok())
                .map(|i| (i - 1) as usize)
                .collect();

            if idx.len() >= 3 {
                for i in 1..idx.len() - 1 {
                    tris.push([idx[0], idx[i], idx[i + 1]]);
                }
            }
        }
    }

    if verts.is_empty() || tris.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No usable data found",
        ));
    }

    Ok((verts, tris))
}

pub fn normalize_mesh(verts: &mut [Vec3]) {
    let n = verts.len() as f32;
    let mut centroid = Vec3::new(0.0, 0.0, 0.0);
    for v in verts.iter() {
        centroid = centroid.add(v);
    }
    centroid = centroid.scale(1.0 / n);

    let mut max_r: f32 = 0.0;
    for v in verts.iter_mut() {
        *v = v.subtract(&centroid);
        max_r = max_r.max(v.length());
    }

    if max_r > 1e-6 {
        for v in verts.iter_mut() {
            *v = v.scale(1.0 / max_r);
        }
    }
}
