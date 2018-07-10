use std::collections::HashMap;

use na;

pub type TilePos = na::Point2<u64>;
pub type PatchPos = TilePos;

pub struct Tile {
}

pub struct Map {
    patch_size: na::Vector2<u64>,
    patches: HashMap<PatchPos, Patch>,
}

struct Patch {
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            patch_size: na::Vector2::new(128, 128),
            patches: HashMap::new(),
        }
    }

    /*pub fn get(&self, tile_pos: TilePos) -> Option<&Tile> {
    }

    fn decompose_pos(tile_pos: TilePos) -> (PatchPos, TilePos) {

    }*/
}
