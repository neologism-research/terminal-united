#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Grass,
    Water,
    Desk,
    CoffeeMachine,
    Void, // Represents "empty" or "out of bounds"
}

pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn load() -> Self {
        let content = include_str!("../assets/world_map.txt");

        let tiles: Vec<Vec<TileType>> = content
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Self::char_to_tile(c)) // Convert char -> Enum immediately
                    .collect()
            })
            .collect();

        let height = tiles.len();
        let width = if height > 0 { tiles[0].len() } else { 0 };

        Self {
            tiles,
            width,
            height,
        }
    }

    // 3. The Parser: The ONLY place that cares about specific characters.
    fn char_to_tile(c: char) -> TileType {
        match c {
            '#' => TileType::Wall,
            '.' => TileType::Floor,
            ',' => TileType::Grass,
            '~' => TileType::Water,
            'D' => TileType::Desk,
            'C' => TileType::CoffeeMachine,
            _ => TileType::Void, // Treat unknown chars as Void
        }
    }

    // Helper to check collision (used later by Player)
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        match self.tiles[y][x] {
            TileType::Wall | TileType::Void | TileType::CoffeeMachine | TileType::Water => false,
            _ => true,
        }
    }
}
