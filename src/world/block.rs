// pub struct Block {
//     x: i32,
//     y: i32,
//     z: i32

//     // TODO: Texure
//     // TODO: Type
// }

#[derive(Clone, Copy)]
pub enum Block {
    None,
    Grass,
    Dirt,
    Water, // TODO: Add more!

           // TODO: Include indices into texture map
}
