use packer::Packer;

#[derive(Packer)]
#[packer(source = "textures", prefixed = false)]
pub struct Textures;

#[derive(Packer)]
#[packer(source = "shaders", prefixed = false)]
pub struct Shaders;
