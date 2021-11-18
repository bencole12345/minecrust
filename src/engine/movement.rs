/// An object that can be translated and rotated
pub trait Moveable {
    fn move_forwards(&mut self, distance: f32);
    fn move_backwards(&mut self, distance: f32);
    fn move_left(&mut self, distance: f32);
    fn move_right(&mut self, distance: f32);
    fn move_up(&mut self, distance: f32);
    fn move_down(&mut self, distance: f32);
    fn rotate_left(&mut self, angle: f32);
    fn rotate_right(&mut self, angle: f32);
    fn rotate_up(&mut self, angle: f32);
    fn rotate_down(&mut self, angle: f32);
}
