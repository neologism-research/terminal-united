use crate::map::Map;
use terminal_united_shared::{DEFAULT_SPAWN_X, DEFAULT_SPAWN_Y};

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub char: char,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y, char: '@' }
    }

    pub fn try_move(&mut self, dx: i32, dy: i32, map: &Map) -> bool {
        let new_x = self.x as i32 + dx;
        let new_y = self.y as i32 + dy;

        if new_x < 0 || new_y < 0 {
            return false;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if map.is_walkable(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
            true
        } else {
            false
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(DEFAULT_SPAWN_X as usize, DEFAULT_SPAWN_Y as usize)
    }
}
