mod helpers;
mod vectors;
use std::thread::sleep;
use std::time::Duration;

mod parser;
use clap::{Parser, ValueEnum};
use helpers::{project, rasterize_flat};
use parser::{normalize_mesh, parse_obj};
use vectors::Vec3;

use terminal_size::{terminal_size, Height, Width};

#[derive(Clone, ValueEnum)]
enum Mode {
    Solid,
    Textured,
}

#[derive(Parser)]
#[command(name = "ravioli")]
struct Args {
    #[arg(short = 'O', long = "obj")]
    obj: Option<String>,

    #[arg(short = 'S', long = "speed", default_value_t = 1.0)]
    speed: f32,

    #[arg(short = 'C', long = "camera-distance", default_value_t = 1.5)]
    camera_distance: f32,

    #[arg(short = 'M', long = "mode", value_enum, default_value_t = Mode::Textured)]
    mode: Mode,
}

fn main() {
    let args = Args::parse();

    let cube_vertices: Vec<Vec3> = vec![
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
    ];

    let cube_triangles: Vec<[usize; 3]> = vec![
        [0, 1, 2],
        [0, 2, 3],
        [5, 4, 7],
        [5, 7, 6],
        [4, 0, 3],
        [4, 3, 7],
        [1, 5, 6],
        [1, 6, 2],
        [3, 2, 6],
        [3, 6, 7],
        [4, 5, 1],
        [4, 1, 0],
    ];

    let (mut cube_vertices, cube_triangles) = match &args.obj {
        Some(path) => parse_obj(path).expect("Failed to load obj file"),
        None => (cube_vertices, cube_triangles),
    };
    normalize_mesh(&mut cube_vertices);

    let char_aspect: f32 = 2.0;
    let fill_fraction: f32 = 0.42;

    let mut angle = Vec3::new(0.0, 0.0, 0.0);
    let increment_angle = Vec3::new(0.02, 0.02, 0.02).scale(args.speed);
    let camera_distance: f32 = args.camera_distance;

    let camera = Vec3::new(0.0, 0.0, camera_distance);
    let ramp: Vec<char> = " .:-=+*#%@".chars().collect();

    loop {
        let Some((Width(cols), Height(rows))) = terminal_size() else {
            panic!("Couldn't retrieve terminal size");
        };
        let scale: f32 = (cols as f32).min(rows as f32 * char_aspect) * fill_fraction;
        let mut grid: Vec<char> = vec![' '; cols as usize * rows as usize];
        let mut zbuf: Vec<f32> = vec![f32::NEG_INFINITY; cols as usize * rows as usize];
        let mut color_buf: Vec<(u8, u8, u8)> = vec![(0, 0, 0); cols as usize * rows as usize];
        let light_color: (f32, f32, f32) = (255.0, 255.0, 255.0);

        for tri in cube_triangles.iter() {
            let rotated = tri
                .iter()
                .map(|&idx| {
                    cube_vertices[idx]
                        .rotate_x(angle.x)
                        .rotate_y(angle.y)
                        .rotate_z(angle.z)
                        .add(&camera)
                })
                .collect::<Vec<Vec3>>();

            let edge1 = rotated[1].subtract(&rotated[0]);
            let edge2 = rotated[2].subtract(&rotated[0]);
            let normal = edge2.cross(&edge1).normalize();

            let light_dir = Vec3::new(-0.5, 0.6, -1.0).normalize();
            let fill_dir = Vec3::new(0.6, -0.3, 0.7).normalize();

            let key = normal.dot(&light_dir).max(0.0);
            let fill = normal.dot(&fill_dir).max(0.0) * 0.35;
            let brightness = (key + fill).clamp(0.0, 1.0) * 0.7 + 0.3;

            let ch = match args.mode {
                Mode::Solid => '█',
                Mode::Textured => {
                    let density_brightness = (brightness * 0.3 + 0.7).clamp(0.0, 1.0);
                    let idx = (density_brightness * (ramp.len() - 1) as f32).round() as usize;
                    ramp[idx]
                }
            };

            let r = (light_color.0 * brightness).clamp(0.0, 255.0) as u8;
            let g = (light_color.1 * brightness).clamp(0.0, 255.0) as u8;
            let b = (light_color.2 * brightness).clamp(0.0, 255.0) as u8;

            let projected: Vec<Option<(f32, f32, f32)>> = rotated
                .iter()
                .map(|v| project(*v, cols as usize, rows as usize, scale, char_aspect))
                .collect();

            let (Some(p0), Some(p1), Some(p2)) = (projected[0], projected[1], projected[2]) else {
                continue;
            };

            rasterize_flat(
                p0,
                p1,
                p2,
                ch,
                (r, g, b),
                cols as usize,
                rows as usize,
                &mut grid,
                &mut zbuf,
                &mut color_buf,
            );
        }

        let mut frame = String::from("\x1b[H");
        let mut last_color: Option<(u8, u8, u8)> = None;

        for y in 0..rows as usize {
            for x in 0..cols as usize {
                let i = y * cols as usize + x;
                let ch = grid[i];
                let color = color_buf[i];

                if ch != ' ' && last_color != Some(color) {
                    frame.push_str(&format!("\x1b[38;2;{};{};{}m", color.0, color.1, color.2));
                    last_color = Some(color);
                }
                frame.push(ch);
            }
            frame.push('\n');
        }
        frame.push_str("\x1b[0m");

        print!("{frame}");
        use std::io::Write;
        std::io::stdout().flush().unwrap();

        angle = angle.add(&increment_angle);
        sleep(Duration::from_millis(50));
    }
}
