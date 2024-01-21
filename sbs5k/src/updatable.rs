// TODO: Think about the name of this. Perhaps something like "OnTickListener" would be better?
/// The central protocol for game objects that expect to perform some logic on each cycle
pub(crate) trait Updatable {
    /// Will be invoked on each iteration of the game loop, for any registered `Updatable`
    fn update(&mut self, dt: f32);
}
