#[derive(Debug, Clone, Copy)]
pub struct BangBangController {
    min_input: f64,
    max_input: f64,
    min_output: f64,
    max_output: f64,
    set_point: f64,
    tolerance: f64,
    enabled: bool,
}

impl BangBangController {
    #[must_use]
    pub const fn new(tolerance: f64) -> Self {
        Self {
            min_input: -1.0,
            max_input: 1.0,
            min_output: -1.0,
            max_output: 1.0,
            set_point: 0.0,
            tolerance,
            enabled: true,
        }
    }

    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Returns the control output.
    #[must_use]
    pub fn calculate(&self, measurement: f64) -> f64 {
        if !self.enabled {
            return 0.0;
        }
        if measurement.clamp(self.min_input, self.max_input) < self.set_point {
            self.max_output
        } else {
            self.min_output
        }
    }

    /// Sets the set point.
    pub fn set_set_point(&mut self, set_point: f64) {
        self.set_point = set_point;
    }

    /// Enables or disables the controller.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns whether the controller is enabled.
    #[must_use]
    pub const fn get_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the set point.
    #[must_use]
    pub const fn get_set_point(&self) -> f64 {
        self.set_point
    }

    /// Sets the input and output limits.
    pub fn set_limits(&mut self, min_input: f64, max_input: f64, min_output: f64, max_output: f64) {
        self.min_input = min_input;
        self.max_input = max_input;
        self.min_output = min_output;
        self.max_output = max_output;
    }

    /// Resets the controller.
    pub fn reset(&mut self) {
        self.set_point = 0.0;
    }
}
