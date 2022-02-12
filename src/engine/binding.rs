/// Indicates an object that can be "bound" and "unbound".
///
/// The idea is to handle binding objects - say, an OpenGL vertex buffer - and
/// subsequently unbinding them in a safer, RAII way.
pub trait Bindable<'a> {
    fn bind(&self);
    fn unbind(&self);
}

/// An RAII guard that automatically binds and unbinds a `Bindable` object
///
/// A bind is constructed using the `Bindable::create_bind()` method, which invokes the bindable
/// object's `bind()` method. The resulting `BindGuard` will then automatically `unbind()` the
/// object upon being dropped.
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
