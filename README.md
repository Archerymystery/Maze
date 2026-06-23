# Maze
## Description 
This project is a maze generator writen in Rust. The maze is generating using [Randomized depth-first search](https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_depth-first_search), which guarantee O(V) time complexity,where V is the total number of cells in the maze grid that can be visited. To optimize trap placement, the maze is first solved using DFS to identify the main path; this ensures the entire process maintains the same O(V) time complexity. 
## Installing dependencies and starting the program
### Linux/Mac os
1. Install rust if not installed 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Clone repo
```bash
git clone https://github.com/Archerymystery/Maze.git
cd Orderbook
```
3. Run the program
```bash
cargo run
```
### Nix/Nixos
If you are using Nix, you can run the program directly from GitHub without cloning:
```bash
nix run github:Archerymystery/Maze
```
### Windows
1. [Install rust](https://rust-lang.org/tools/install/)
2. Clone repo
```
git clone https://github.com/Archerymystery/Maze.git
cd Orderbook
```
3. Run the program
```
cargo run 
```

## Time Spent 
Around 5 hours of work 

