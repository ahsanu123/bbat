use design_pattern::factory_method_maze::*;

use magic_maze::MagicMaze;
use ordinary_maze::OrdinaryMaze;

fn main() {
    let ordinary_maze = OrdinaryMaze::default();
    game::run(ordinary_maze);

    println!("=====================");

    let magic_maze = MagicMaze::default();
    game::run(magic_maze);
}
