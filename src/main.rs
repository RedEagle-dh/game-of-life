use minifb::{Window, WindowOptions};
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        print_usage(&args[0]);
        return;
    }

    let window_width = parse_arg(&args, 1, 1000);
    let window_height = parse_arg(&args, 2, 1000);
    let grid_width = parse_arg(&args, 3, 64);
    let grid_height = parse_arg(&args, 4, 64);

    let mut grid = generate_grid(grid_width, grid_height, 1000);

    let mut window = match Window::new(
        "Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    window.limit_update_rate(Some(std::time::Duration::from_millis(100)));

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        grid = run_simulation_step(grid);
        let buffer = create_pixel_buffer(&grid, window_width, window_height);
        window
            .update_with_buffer(&buffer, window_width, window_height)
            .unwrap();
    }
}

fn generate_grid(width: usize, height: usize, seed: u64) -> Vec<Vec<bool>> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut grid = vec![vec![false; width]; height];
    for y in 0..height {
        for x in 0..width {
            grid[y][x] = rng.gen_bool(0.5);
        }
    }
    grid
}

fn create_pixel_buffer(
    grid: &Vec<Vec<bool>>,
    window_width: usize,
    window_height: usize,
) -> Vec<u32> {
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let cell_width = window_width / grid_width;
    let cell_height = window_height / grid_height;

    let mut buffer = vec![0u32; window_width * window_height];

    for y in 0..grid_height {
        for x in 0..grid_width {
            //let color = if grid[y][x] { 0x00FFFFFF } else { 0x00000000 };
            for py in 0..cell_height {
                for px in 0..cell_width {
                    let color = if px == 0 || py == 0 {
                        0x00333333 // dunkelgrau als Border
                    } else if grid[y][x] {
                        0x00FFFFFF
                    } else {
                        0x00000000
                    };
                    let pixel_index = (y * cell_height + py) * window_width + (x * cell_width + px);
                    buffer[pixel_index] = color;
                }
            }
        }
    }

    buffer
}

fn run_simulation_step(mut grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let read_grid = grid.clone();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let neighbours_counter = calculate_rule(&read_grid, x, y);
            if neighbours_counter == 3 {
                grid[y][x] = true;
            } else if neighbours_counter == 2 {
                grid[y][x] = grid[y][x];
            } else {
                grid[y][x] = false;
            }
        }
    }
    return grid;
}

fn calculate_rule(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> i32 {
    let mut neighbours_counter = 0;
    let mut min_x_neighbour_idx = x;
    let mut min_y_neighbour_idx = y;
    let mut max_x_neighbour_idx = x;
    let mut max_y_neighbour_idx = y;

    if x > 0 {
        min_x_neighbour_idx = x - 1;
    }
    if x < grid[y].len() - 1 {
        max_x_neighbour_idx = x + 1;
    }

    if y > 0 {
        min_y_neighbour_idx = y - 1;
    }

    if y < grid.len() - 1 {
        max_y_neighbour_idx = y + 1;
    }

    for yi in min_y_neighbour_idx..=max_y_neighbour_idx {
        for xi in min_x_neighbour_idx..=max_x_neighbour_idx {
            if yi > grid.len() || xi > grid[y].len() {
                continue;
            } else if yi == y && xi == x {
                continue;
            } else {
                if grid[yi][xi] == true {
                    neighbours_counter += 1;
                }
            }
        }
    }
    return neighbours_counter;
}

fn parse_arg(args: &[String], index: usize, default: usize) -> usize {
    args.get(index)
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn print_usage(program: &str) {
    println!("Conway's Game of Life");
    println!();
    println!("Usage: {} [window_width] [window_height] [grid_width] [grid_height]", program);
    println!();
    println!("Arguments:");
    println!("  window_width   Window width in pixels (default: 1000)");
    println!("  window_height  Window height in pixels (default: 1000)");
    println!("  grid_width     Grid width in cells (default: 64)");
    println!("  grid_height    Grid height in cells (default: 64)");
    println!();
    println!("Examples:");
    println!("  {}                      # Use all defaults", program);
    println!("  {} 800 600              # 800x600 window, 64x64 grid", program);
    println!("  {} 800 600 32 32        # 800x600 window, 32x32 grid", program);
}
