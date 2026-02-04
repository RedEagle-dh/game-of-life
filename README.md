# Conway's Game of Life

A Rust implementation of Conway's Game of Life with a graphical display using minifb.

## About

This is a cellular automaton simulation following Conway's classic rules:

1. **Birth**: A dead cell with exactly 3 live neighbors becomes alive
2. **Survival**: A live cell with 2 or 3 live neighbors survives
3. **Death**: All other cells die or stay dead

The simulation runs on a 64x64 grid with a randomized initial state.

## Requirements

- Rust toolchain (edition 2024)
- Cargo package manager

## Clone

```bash
git clone https://github.com/RedEagle-dh/game-of-life.git
cd game-of-life
```

## Build & Run

```bash
# Run with defaults (1000x1000 window, 64x64 grid)
cargo run

# Or optimized release build
cargo run --release
```

## Usage

```
game-of-life [window_width] [window_height] [grid_width] [grid_height]
```

| Argument | Default | Description |
|----------|---------|-------------|
| `window_width` | 1000 | Window width in pixels |
| `window_height` | 1000 | Window height in pixels |
| `grid_width` | 64 | Grid width in cells |
| `grid_height` | 64 | Grid height in cells |

### Examples

```bash
# Use all defaults
cargo run

# 800x600 window with default 64x64 grid
cargo run -- 800 600

# 800x600 window with 32x32 grid
cargo run -- 800 600 32 32

# Show help
cargo run -- --help
```

## Controls

- **Escape**: Exit the simulation

The simulation runs at 10 generations per second (100ms per tick).

## License

MIT
