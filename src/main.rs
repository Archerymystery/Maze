use colored::*;
use dialoguer::{Input, theme::ColorfulTheme};
use rand::{random_bool, random_range, seq::IndexedMutRandom};
use std::{f64::consts::PI, vec};
#[derive(Debug, Clone, Copy, PartialEq)]
enum MazeSquare {
    Entrance,
    Exit,
    Road,
    Wall,
    Trap,
    Treasure,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
struct Maze {
    width: usize,
    height: usize,
    data: Vec<MazeSquare>,
}
impl Maze {
    fn set_point(&mut self, p: Point, value: MazeSquare) {
        if p.x < self.width && p.y < self.height {
            let index = p.y * self.width + p.x;
            self.data[index] = value;
        }
    }
    fn get_point(&mut self, p: Point) -> Option<MazeSquare> {
        if p.x < self.width && p.y < self.height {
            let index = p.y * self.width + p.x;
            return Some(self.data[index]);
        }
        None
    }
    fn print(&self) {
        println!("{} - Entrance\n", "██".green());
        println!("{} - Exit\n", "██".blue());
        println!("{} - Trap\n", "██".red());
        println!("{} - Treasure\n", "██".yellow());
        for row in 0..self.height {
            for col in 0..self.width {
                let index = row * self.width + col;
                match self.data[index] {
                    MazeSquare::Entrance => print!("{}", "██".green()),
                    MazeSquare::Exit => print!("{}", "██".blue()),
                    MazeSquare::Road => print!("  "),
                    MazeSquare::Wall => print!("██"),
                    MazeSquare::Trap => print!("{}", "██".red()),
                    MazeSquare::Treasure => print!("{}", "██".yellow()),
                }
            }
            println!();
        }
    }
}

fn generate_maze(mut width: usize, mut height: usize) -> Maze {
    let mut rng = rand::rng();
    let idx = move |p: Point| p.y * width + p.x;
    if height.is_multiple_of(2) {
        height -= 1;
    }
    if width.is_multiple_of(2) {
        width -= 1;
    }
    let mut maze = Maze {
        width,
        height,
        data: vec![MazeSquare::Wall; width * height],
    };

    let count = (height / 2) * (width / 2);
    let mut visited_count: usize = 1;
    let mut visited = vec![false; width * height];
    let mut stack: Vec<Point> = vec![];
    let start_point = Point {
        x: 1 + random_range(1..=(width - 2) / 2) * 2,
        y: 1 + random_range(1..=(height - 2) / 2) * 2,
    };
    stack.push(start_point);
    visited[idx(start_point)] = true;
    maze.set_point(start_point, MazeSquare::Entrance);

    let mut available_points: Vec<Point> = Vec::new();
    let mut max_path_len = 1;
    let mut exit_point: Point = Point { x: 1, y: 1 };
    let mut is_treasure = false;
    let mut is_backtarck = false;
    if random_bool(0.5) {
        is_treasure = true;
    }
    loop {
        available_points.clear();
        let current = *stack.last().unwrap();

        let directions = [(0, -2), (0, 2), (-2, 0), (2, 0)];

        for (dx, dy) in directions {
            let nx = current.x as isize + dx;
            let ny = current.y as isize + dy;

            if nx > 0 && nx < (width - 1) as isize && ny > 0 && ny < (height - 1) as isize {
                let next_point = Point {
                    x: nx as usize,
                    y: ny as usize,
                };

                if !visited[idx(next_point)] {
                    available_points.push(next_point);
                }
            }
        }
        if available_points.is_empty() {
            stack.pop();
            if random_bool((PI * (visited_count as f64 + 2.0) / (1.3 * count as f64)).sin() / 2.0)
                && !is_treasure
                && !is_backtarck
            {
                is_treasure = true;

                maze.set_point(current, MazeSquare::Treasure);
            }
            is_backtarck = true;
            continue;
        }
        is_backtarck = false;
        let point = available_points.choose_mut(&mut rng).unwrap();
        maze.set_point(
            Point {
                x: (point.x + current.x) / 2,
                y: (point.y + current.y) / 2,
            },
            MazeSquare::Road,
        );
        maze.set_point(*point, MazeSquare::Road);
        stack.push(*point);
        if stack.len() > max_path_len {
            max_path_len = stack.len();
            exit_point = *point;
        }
        visited[idx(*point)] = true;

        visited_count += 1;
        if visited_count == count {
            break;
        }
    }

    maze.set_point(exit_point, MazeSquare::Exit);
    visited.fill(false);
    stack.clear();
    stack.push(start_point);
    visited[idx(start_point)] = true;
    loop {
        available_points.clear();
        let current = *stack.last().unwrap();
        if current == exit_point {
            break;
        }
        let directions = [(0, -2), (0, 2), (-2, 0), (2, 0)];

        for (dx, dy) in directions {
            let nx = current.x as isize + dx;
            let ny = current.y as isize + dy;

            if nx > 0 && nx < (width - 1) as isize && ny > 0 && ny < (height - 1) as isize {
                let next_point = Point {
                    x: nx as usize,
                    y: ny as usize,
                };

                if !visited[idx(next_point)]
                    && maze.get_point(Point {
                        x: (current.x + next_point.x) / 2,
                        y: (current.y + next_point.y) / 2,
                    }) != Some(MazeSquare::Wall)
                {
                    available_points.push(next_point);
                }
            }
        }
        if available_points.is_empty() {
            maze.set_point(stack.pop().unwrap(), MazeSquare::Road);
            continue;
        }

        let point = available_points.choose_mut(&mut rng).unwrap();

        stack.push(*point);
        visited[idx(*point)] = true;
    }
    let mut traps_on_main = 0;
    for _ in 0..=random_range(0..=5) {
        let trap_point = Point {
            x: 1 + random_range(1..=(width - 2) / 2) * 2,
            y: 1 + random_range(1..=(height - 2) / 2) * 2,
        };
        if maze.get_point(trap_point) != Some(MazeSquare::Wall) {
            if stack.contains(&trap_point) && traps_on_main <= 2 {
                traps_on_main += 1;
                maze.set_point(trap_point, MazeSquare::Trap);
            } else {
                maze.set_point(trap_point, MazeSquare::Trap);
            }
        }
    }
    maze
}
fn main() {
    let theme = ColorfulTheme::default();
    let width: usize = Input::with_theme(&theme)
        .with_prompt("Enter maze width (even numbers will be reduced by 1)")
        .interact_text()
        .unwrap();

    let height: usize = Input::with_theme(&theme)
        .with_prompt("Enter maze height (even numbers will be reduced by 1)")
        .interact_text()
        .unwrap();
    let maze = generate_maze(width, height);
    maze.print();
}
