use serde::{Deserialize, Serialize};

/// The number of non-empty blocks included in the `Block` enum.
///
/// This is used by the rendering process for dividing the block textures file into segments. If
/// there are `N` textured blocks whose textures are included in the file, then the `i`th's texture
/// will range from (zero-based) width `i/N` to `(i+1)/N`.
// TODO: Set this properly once we have textures for the other block types
pub const NON_EMPTY_BLOCKS_COUNT: u32 = 3;

/// Encodes all possible block types in the game world.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum Block {
    #[default]
    Empty = 0,
    Grass = 1,
    Dirt = 2,
    Stone = 3,
    // TODO: Water
    // TODO: Torch
}
