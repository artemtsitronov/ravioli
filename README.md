# ravioli

An ASCII 3D OBJ viewer. Animations, and RGB-colored shading is supported.
The only external dependencies are `clap` for CLI parsing and `terminal_size` for live terminal sizing.

## Features

- OBJ mesh loading (vertices + n-gon faces, fan-triangulated)
- Perspective projection with a software rasterizer (barycentric fill + z-buffer)
- Two-light (key + fill) flat shading with true 24-bit RGB color
- Backface culling
- Solid or textured (ASCII ramp) render modes
- Live terminal-resize support
- Falls back to a built-in demo cube if no file is given

## Install

```bash
cargo install ravioli
```

## Usage

```bash
ravioli                              # spin the built-in demo cube
ravioli -O model.obj                 # spin your own mesh
ravioli -O model.obj -S 2.0          # faster rotation
ravioli -O model.obj -C 3.0          # camera further back
ravioli -O model.obj -M solid        # solid block glyphs
ravioli -O model.obj -R false        # disable live terminal-resize
```

| Flag | Long form | Default | Description |
|---|---|---|---|
| `-O` | `--obj` | *(built-in cube)* | Path to an `.obj` file |
| `-S` | `--speed` | `1.0` | Rotation speed multiplier |
| `-C` | `--camera-distance` | `1.5` | Distance of the camera from the object |
| `-M` | `--mode` | `textured` | `solid` or `textured` |
| `-R` | `--resize` | `true` | Re-detect terminal size live while running |
