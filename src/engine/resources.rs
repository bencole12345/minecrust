use packer::Packer;

#[derive(Packer)]
#[packer(source = "shaders", prefixed = false)]
struct Shaders;

#[inline]
fn get_resource(name: &'static str) -> &'static [u8] {
    Shaders::get(name).unwrap()
}

pub fn scene_objects_vertex_shader() -> (&'static [u8], &'static str) {
    let name = "scene_objects.vert";
    (get_resource(name), name)
}

pub fn scene_objects_fragment_shader() -> (&'static [u8], &'static str) {
    let name = "scene_objects.frag";
    (get_resource(name), name)
}

pub fn skybox_vertex_shader() -> (&'static [u8], &'static str) {
    let name = "skybox.vert";
    (get_resource(name), name)
}

pub fn skybox_fragment_shader() -> (&'static [u8], &'static str) {
    let name = "skybox.frag";
    (get_resource(name), name)
}
