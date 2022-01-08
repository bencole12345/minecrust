use glfw;

/// Tool to measure the time difference between successive frames.
///
/// The owner is required to call `tick()` each frame.
pub struct TimeTracker {
    time_prev_frame: Option<f64>,
    time_curr_frame: f64,
    target_fps: Option<f32>,
}

impl TimeTracker {
    pub fn new() -> Self {
        TimeTracker {
            time_prev_frame: None,
            time_curr_frame: get_time(),
            target_fps: None,
        }
    }

    /// Update the internal clock from the previous frame
    pub fn tick(&mut self) {
        self.time_prev_frame = Some(self.time_curr_frame);
        self.time_curr_frame = get_time();
    }

    // TODO: Implement tick(target_fps: u32) that'll sleep/yield/? until dt >= 1/target_fps

    /// Compute the amount of time that elapsed since the previous frame, in seconds
    pub fn dt(&self) -> f64 {
        self.time_curr_frame - self.time_prev_frame.unwrap()
    }
}

fn get_time() -> f64 {
    unsafe { glfw::ffi::glfwGetTime() }
}
