// TODO: Set this properly once we have textures for the other block types
pub const NON_EMPTY_BLOCKS_COUNT: u32 = 1;

/// Encodes all possible block types in the game world.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Block {
    Empty,
    Grass,
    Dirt,
    Stone,
    // TODO: Water
    // TODO: Torch
    // TODO: Include indices into texture map
}

impl Default for Block {
    fn default() -> Self {
        Block::Empty
    }
}
