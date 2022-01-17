use packer::Packer;

#[derive(Packer)]
#[packer(source = "textures", prefixed = false)]
struct Textures;

pub fn cubes_texture() -> &'static [u8] {
    Textures::get("cubes.png").unwrap()
}
