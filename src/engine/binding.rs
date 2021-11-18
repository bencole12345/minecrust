/// Indicates an object that can be "bound" and "unbound".
///
/// The idea is to handle binding objects - say, an OpenGL vertex buffer - and
/// subsequently unbinding them in a safer, RAII way.
pub trait Bindable<'a> {
    fn bind(&self);
    fn unbind(&self);
}

pub struct BindGuard<'a, T: Bindable<'a>> {
    bound_target: &'a T,
}

impl<'a, T: Bindable<'a>> BindGuard<'a, T> {
    pub fn create_bind(target: &'a T) -> BindGuard<'a, T> {
        target.bind();
        BindGuard {
            bound_target: target,
        }
    }
}

impl<'a, T: Bindable<'a>> Drop for BindGuard<'a, T> {
    fn drop(&mut self) {
        self.bound_target.unbind();
    }
}
