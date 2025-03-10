pub trait Room {
    fn render(&self);
}

pub trait MazeGame {
    type RoomImpl: Room;

    fn rooms(&self) -> Vec<Self::RoomImpl>;

    fn play(&self) {
        for room in self.rooms() {
            room.render();
        }
    }
}

pub fn run(maze_game: impl MazeGame) {
    println!("Loading Resource...");
    println!("Starting the Game...");

    maze_game.play();
}
