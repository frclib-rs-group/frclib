use frclib_core::units::time::Millisecond;

#[derive(Debug, Clone, Copy)]
pub struct PIDController {
    pub k_p: f64,
    pub k_d: f64,
    pub min_input: f64,
    pub max_input: f64,
    pub min_output: f64,
    pub max_output: f64,
    prev_error: f64,
    total_error: f64,
    target: f64,
    enabled: bool,
}

impl PIDController {
    #[must_use]
    pub const fn new(k_p: f64, k_d: f64) -> Self {
        Self {
            k_p,
            k_d,
            min_input: -1.0,
            max_input: 1.0,
            min_output: -1.0,
            max_output: 1.0,
            prev_error: 0.0,
            total_error: 0.0,
            target: 0.0,
            enabled: true,
        }
    }

    pub fn calculate(&mut self, measurement: f64, period: impl Into<Millisecond>) -> f64 {
        let period: f64 = period.into().value();
        if !self.enabled {
            return 0.0;
        }
        let error = self.target - measurement;
        self.total_error += error * period;
        let d_error = (error - self.prev_error) / period;
        self.prev_error = error;
        let p = self.k_p * error;
        let d = self.k_d * d_error;
        let output = p + d;
        output.clamp(self.min_output, self.max_output)
    }

    pub fn set_target(&mut self, set_point: f64) {
        self.target = set_point;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    #[must_use]
    pub const fn get_enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub const fn get_target(&self) -> f64 {
        self.target
    }

    pub fn set_limits(&mut self, min_input: f64, max_input: f64, min_output: f64, max_output: f64) {
        self.min_input = min_input;
        self.max_input = max_input;
        self.min_output = min_output;
        self.max_output = max_output;
    }

    pub fn reset(&mut self) {
        self.prev_error = 0.0;
        self.total_error = 0.0;
    }
}
