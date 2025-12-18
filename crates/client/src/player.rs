use crate::map::Map;

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub char: char, // The visual representation (e.g., '@')
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y, char: '@' }
    }

    /// Try to move by delta. Returns true if the move was successful.
    pub fn try_move(&mut self, dx: i32, dy: i32, map: &Map) -> bool {
        // 1. Calculate Target Coordinates
        // We cast to i32 first to handle negative math (moving left/up),
        // then safely cast back to usize.
        let new_x = self.x as i32 + dx;
        let new_y = self.y as i32 + dy;

        // 2. Safety Check: Negative Bounds
        // If moving would result in negative coordinates, stop immediately.
        if new_x < 0 || new_y < 0 {
            return false;
        }

        // 3. Cast to usize for Map Lookup
        let new_x = new_x as usize;
        let new_y = new_y as usize;

        // 4. Collision Check
        // Ask the Map module: "Is this tile solid?"
        if map.is_walkable(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
            true
        } else {
            false
        }
    }
}
