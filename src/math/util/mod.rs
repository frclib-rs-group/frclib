use nalgebra::ComplexField;

#[must_use]
pub fn apply_deadband(value: f64, deadband: f64, max_magnitude: f64) -> f64 {
    if ComplexField::abs(value) > deadband {
        if max_magnitude / deadband > 1.0e12 {
            if value > 0.0 {
                return value - deadband;
            }
            return value + deadband;
        }
        if value > 0.0 {
            max_magnitude * (value - deadband) / (max_magnitude - deadband)
        } else {
            max_magnitude * (value + deadband) / (max_magnitude - deadband)
        }
    } else {
        0.0
    }
}


#[must_use]
pub fn apply_deadband_no_max(value: f64, deadband: f64) -> f64 {
    apply_deadband(value, deadband, 1.0)
}

#[must_use]
pub fn input_modulus(input: f64, minimum_input: f64, maximum_input: f64) -> f64 {
    let modulus = maximum_input - minimum_input;

    let num_max = (input - minimum_input) / modulus;
    let input = num_max.floor().mul_add(-modulus, input);

    let num_min = (input - maximum_input) / modulus;
    num_min.floor().mul_add(-modulus, input)
}

#[must_use]
pub fn angle_modulus(angle_radians: f64) -> f64 {
    input_modulus(angle_radians, -std::f64::consts::PI, std::f64::consts::PI)
}

#[must_use]
#[allow(clippy::expect_used)]
pub(crate) fn interpolate<F: num::Float>(start_value: F, end_value: F, t: f64) -> F {
    (end_value - start_value)
        .mul_add(
            F::from(t.clamp(0.0, 1.0)).expect("Failed to convert f64 to F"),
            start_value
        )
}

#[must_use]
pub fn is_near(expected: f64, actual: f64, tolerance: f64) -> bool {
    if tolerance < 0.0 {
        false
    } else {
        ComplexField::abs(expected - actual) < tolerance
    }
}

#[must_use]
pub fn is_near_min_max(expected: f64, actual: f64, tolerance: f64, min: f64, max: f64) -> bool {
    if tolerance < 0.0 {
        false
    } else {
        // Max error is exactly halfway between the min and max
        let error_bound = (max - min) / 2.0;
        let error = input_modulus(expected - actual, -error_bound, error_bound);
        ComplexField::abs(error) < tolerance
    }
}
