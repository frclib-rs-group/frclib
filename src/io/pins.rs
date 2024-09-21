use std::fmt::Debug;

use frclib_core::hal::{
    get_hal,
    gpio::{
        analog::AnalogInput, digital::{DigitalInput, DigitalOutput}, GPIOError
    },
};

pub struct DigitalIn {
    inner: DigitalInput,
}

impl Debug for DigitalIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DigitalIn")
            .field("channel", &self.inner.channel_id())
            .field("value", &self.inner.read())
            .finish()
    }
}

impl std::fmt::Display for DigitalIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DigitalIn({})", self.inner.channel_id())
    }
}

impl DigitalIn {
    /// Create a new ``DigitalIn`` instance for the given channel.
    ///
    /// # Panics
    /// Will panic if called before the [`HAL`](frclib_core::hal) has been initialized.
    ///
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for digital in use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    pub fn try_new(channel: u8) -> Result<Self, GPIOError> {
        Ok(Self {
            inner: get_hal()
                .expect("Tried creating gpio::DigitalInput before HAL was initialized")
                .gpio_api()
                .new_digital_input(channel)?,
        })
    }

    #[must_use]
    /// Will read the value of the digital input
    pub fn read(&self) -> bool {
        self.inner.read()
    }
}

pub struct DigitalOut {
    inner: DigitalOutput,
}

impl Debug for DigitalOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DigitalOut")
            .field("channel", &self.inner.channel_id())
            .finish()
    }
}

impl std::fmt::Display for DigitalOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DigitalOut({})", self.inner.channel_id())
    }
}

impl DigitalOut {
    /// Create a new ``DigitalOut`` instance for the given channel.
    ///
    /// # Panics
    /// Will panic if called before the [`HAL`](frclib_core::hal) has been initialized.
    ///
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for digital out use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    pub fn try_new(channel: u8) -> Result<Self, GPIOError> {
        Ok(Self {
            inner: get_hal()
                .expect("Tried creating gpio::DigitalOutput before HAL was initialized")
                .gpio_api()
                .new_digital_output(channel)?,
        })
    }

    /// Will set the value of the digital output
    pub fn set(&mut self, value: bool) {
        self.inner.write(value);
    }
}


pub struct AnalogIn {
    inner: AnalogInput
}

impl Debug for AnalogIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalogIn")
            .field("channel", &self.inner.channel_id())
            .field("value", &self.inner.read_volts())
            .finish()
    }
}

impl std::fmt::Display for AnalogIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnalogIn({})", self.inner.channel_id())
    }
}

impl AnalogIn {
    /// Create a new ``AnalogIn`` instance for the given channel.
    ///
    /// # Panics
    /// Will panic if called before the [`HAL`](frclib_core::hal) has been initialized.
    ///
    /// # Errors
    /// - [`GPIOError::PortNotAvailable`] if the port is not available for analog in use
    /// - [`GPIOError::PortInUse`] if the port is already in use
    pub fn try_new(channel: u8) -> Result<Self, GPIOError> {
        Ok(Self {
            inner: get_hal()
                .expect("Tried creating gpio::AnalogInput before HAL was initialized")
                .gpio_api()
                .new_analog_input(channel)?,
        })
    }

    #[must_use]
    /// Will read the value of the analog input
    pub fn read(&self) -> f64 {
        self.inner.read_volts().into()
    }
}