pub trait Game {
    /// Perform any required initial setup before the main loop
    fn setup();

    /// Invoked on every main loop iteration
    fn on_tick();

    /// Invoked on each event from the game engine
    fn on_event();

    /// Invoked when the game is finished
    fn on_exit();
}

pub struct Driver<G: Game> {
    game: G,
}

impl<G: Game> Driver<G> {
    pub fn new() -> Self {
        todo!()
    }
}
