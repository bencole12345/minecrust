/// An object that can be translated and rotated
pub trait Moveable {
    /// Move `distance` units forwards
    fn move_forwards(&mut self, distance: f32);

    /// Move `distance` units backwards
    fn move_backwards(&mut self, distance: f32);

    /// Move `distance` units to the left
    fn move_left(&mut self, distance: f32);

    /// Move `distance` units to the right
    fn move_right(&mut self, distance: f32);

    /// Move `distance` units upwards
    fn move_up(&mut self, distance: f32);

    /// Move `distance` units downwards
    fn move_down(&mut self, distance: f32);

    /// Rotate `angle` radians anticlockwise
    fn rotate_left(&mut self, angle: f32);

    /// Rotate `angle` radians clockwise
    fn rotate_right(&mut self, angle: f32);

    /// Rotate `angle` radians upwards
    fn rotate_up(&mut self, angle: f32);

    /// Rotate `angle` radians downwards
    fn rotate_down(&mut self, angle: f32);
}
