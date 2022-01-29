// TODO: Set this properly once we have textures for the other block types
pub const NON_EMPTY_BLOCKS_COUNT: u32 = 3;

/// Encodes all possible block types in the game world.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    Empty,
    Grass,
    Dirt,
    Stone,
    // TODO: Water
    // TODO: Torch
}

impl Default for Block {
    fn default() -> Self {
        Block::Empty
    }
}
