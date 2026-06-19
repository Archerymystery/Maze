use colored::*;
#[derive(Debug, Clone)]
enum MazeSquare {
    Entrance,
    Exit,
    Road,
    Wall,
    Trap,
    Treasure,
}
struct Maze {
    width: usize,
    height: usize,
    data: Vec<MazeSquare>,
}
fn generate_maze(width: usize, height: usize) -> Maze {
    let mut data: Vec<MazeSquare> = vec![MazeSquare::Wall; width * height];

    Maze {
        width,
        height,
        data,
    }
}
fn print_maze(maze: Maze) {
    for row in 0..maze.height {
        for col in 0..maze.width {
            let index = row * maze.width + col;
            match maze.data[index] {
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
fn main() {
    let maze = generate_maze(10, 10);
    print_maze(maze);
}
