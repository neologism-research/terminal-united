#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Grass,
    Water,
    Desk,
    CoffeeMachine,
    Void,
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
            .map(|line| line.chars().map(Self::char_to_tile).collect())
            .collect();

        let height = tiles.len();
        let width = tiles.first().map_or(0, |row| row.len());

        Self {
            tiles,
            width,
            height,
        }
    }

    fn char_to_tile(c: char) -> TileType {
        match c {
            '#' => TileType::Wall,
            '.' => TileType::Floor,
            ',' => TileType::Grass,
            '~' => TileType::Water,
            'D' => TileType::Desk,
            'C' => TileType::CoffeeMachine,
            _ => TileType::Void,
        }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        !matches!(
            self.tiles[y][x],
            TileType::Wall | TileType::Void | TileType::CoffeeMachine | TileType::Water
        )
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::load()
    }
}
