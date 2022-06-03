/// Tool to measure the time difference between successive frames
///
/// The owner is required to call `tick()` each frame.
pub struct TimeTracker {
    time_prev_frame: Option<f64>,
    time_curr_frame: f64,
    _target_fps: Option<f32>,
}

impl TimeTracker {
    pub fn new() -> Self {
        TimeTracker {
            time_prev_frame: None,
            time_curr_frame: get_time(),
            _target_fps: None,
        }
    }

    /// Update the internal clock from the previous frame
    pub fn tick(&mut self) {
        // TODO: Use target_fps if it's been set
        self.time_prev_frame = Some(self.time_curr_frame);
        self.time_curr_frame = get_time();
    }

    /// Compute the amount of time that elapsed since the previous frame, in seconds
    pub fn dt(&self) -> f64 {
        self.time_curr_frame - self.time_prev_frame.unwrap()
    }
}

impl Default for TimeTracker {
    fn default() -> Self {
        TimeTracker::new()
    }
}

fn get_time() -> f64 {
    unsafe { glfw::ffi::glfwGetTime() }
}
