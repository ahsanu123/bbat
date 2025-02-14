use super::game::{MazeGame, Room};

#[derive(Clone)]
pub struct MagicRoom {
    title: String,
}

impl MagicRoom {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl Room for MagicRoom {
    fn render(&self) {
        println!("Magic Room {}", self.title);
    }
}

pub struct MagicMaze {
    rooms: Vec<MagicRoom>,
}

impl Default for MagicMaze {
    fn default() -> Self {
        Self {
            rooms: vec![
                MagicRoom::new("infinite Room".into()),
                MagicRoom::new("Red Room".into()),
            ],
        }
    }
}

impl MazeGame for MagicMaze {
    type RoomImpl = MagicRoom;

    fn rooms(&self) -> Vec<Self::RoomImpl> {
        self.rooms.clone()
    }
}
