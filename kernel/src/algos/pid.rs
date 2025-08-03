// PID controller implementation for embedded/firmware use

#[allow(clippy::upper_case_acronyms)]
pub struct PID {
    pub constants: PIDConstants,
    pub state: PIDState,
    pub limits: PIDLimits,
}

pub struct PIDConstants {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
}

pub struct PIDState {
    pub prev_error: f32,
    pub integral: f32,
}

pub struct PIDLimits {
    pub output_min: f32,
    pub output_max: f32,
}

impl PID {
    pub fn new(constants: PIDConstants, limits: PIDLimits) -> Self {
        Self {
            constants,
            state: PIDState {
                prev_error: 0.0,
                integral: 0.0,
            },
            limits,
        }
    }

    pub fn update(&mut self, dt: f32, desired: f32, measurement: f32) -> f32 {
        let error = desired - measurement;
        self.state.integral += error * dt;
        // Anti-windup: clamp the integral term
        self.state.integral = self
            .state
            .integral
            .clamp(self.limits.output_min, self.limits.output_max);
        let derivative = if dt > 0.0 {
            (error - self.state.prev_error) / dt
        } else {
            0.0
        };
        let output = self.constants.kp * error
            + self.constants.ki * self.state.integral
            + self.constants.kd * derivative;
        let output = output.clamp(self.limits.output_min, self.limits.output_max);
        self.state.prev_error = error;
        output
    }

    pub fn reset(&mut self) {
        self.state.prev_error = 0.0;
        self.state.integral = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_limits() -> PIDLimits {
        PIDLimits {
            output_min: -100.0,
            output_max: 100.0,
        }
    }

    #[test]
    fn test_pid_zero_error() {
        let mut pid = PID::new(
            PIDConstants {
                kp: 1.0,
                ki: 0.0,
                kd: 0.0,
            },
            default_limits(),
        );
        let output = pid.update(1.0, 10.0, 10.0);
        assert_eq!(output, 0.0);
    }

    #[test]
    fn test_pid_proportional() {
        let mut pid = PID::new(
            PIDConstants {
                kp: 2.0,
                ki: 0.0,
                kd: 0.0,
            },
            default_limits(),
        );
        let output = pid.update(1.0, 10.0, 8.0);
        assert_eq!(output, 4.0);
    }

    #[test]
    fn test_pid_integral() {
        let mut pid = PID::new(
            PIDConstants {
                kp: 0.0,
                ki: 1.0,
                kd: 0.0,
            },
            default_limits(),
        );
        let output1 = pid.update(1.0, 10.0, 8.0);
        let output2 = pid.update(1.0, 10.0, 8.0);
        assert_eq!(output1, 2.0);
        assert_eq!(output2, 4.0);
    }

    #[test]
    fn test_pid_derivative() {
        let mut pid = PID::new(
            PIDConstants {
                kp: 0.0,
                ki: 0.0,
                kd: 1.0,
            },
            default_limits(),
        );
        let _output1 = pid.update(1.0, 10.0, 8.0);
        let output2 = pid.update(1.0, 10.0, 9.0);
        assert_eq!(output2, -1.0);
    }

    #[test]
    fn test_pid_clamping() {
        let mut pid = PID::new(
            PIDConstants {
                kp: 100.0,
                ki: 0.0,
                kd: 0.0,
            },
            PIDLimits {
                output_min: -10.0,
                output_max: 10.0,
            },
        );
        let output = pid.update(1.0, 10.0, 0.0);
        assert_eq!(output, 10.0);
    }

    #[test]
    fn test_pid_reset() {
        let mut pid = PID::new(
            PIDConstants {
                kp: 1.0,
                ki: 1.0,
                kd: 1.0,
            },
            default_limits(),
        );
        let _output = pid.update(1.0, 10.0, 8.0);
        pid.reset();
        assert_eq!(pid.state.prev_error, 0.0);
        assert_eq!(pid.state.integral, 0.0);
    }

    #[test]
    fn test_pid_setpoint_change() {
        let mut pid = PID::new(
            PIDConstants {
                kp: 1.0,
                ki: 0.0,
                kd: 0.0,
            },
            default_limits(),
        );
        let output1 = pid.update(1.0, 10.0, 8.0);
        let output2 = pid.update(1.0, 20.0, 8.0);
        assert_eq!(output1, 2.0);
        assert_eq!(output2, 12.0);
    }

    #[test]
    fn test_pid_integral_windup() {
        let mut pid = PID::new(
            PIDConstants {
                kp: 0.0,
                ki: 1.0,
                kd: 0.0,
            },
            PIDLimits {
                output_min: -5.0,
                output_max: 5.0,
            },
        );
        // Apply a large, constant error over several steps
        for _ in 0..20 {
            pid.update(1.0, 10.0, 0.0);
        }
        // The integral term should be clamped to 5.0
        assert_eq!(pid.state.integral, 5.0);
        // Output should also be clamped
        let output = pid.update(1.0, 10.0, 0.0);
        assert_eq!(output, 5.0);
    }
}
