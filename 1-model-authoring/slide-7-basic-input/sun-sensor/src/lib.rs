use nexosim::model::Model;
use serde::{Deserialize, Serialize};

const POWER_ON_VOLTAGE: f32 = 5.0;

#[derive(Serialize, Deserialize)]
pub struct SunSensor {
    voltage: f32,
}

#[Model]
impl SunSensor {
    pub fn voltage_in(&mut self, value: f32) {
        if self.voltage < POWER_ON_VOLTAGE && value >= POWER_ON_VOLTAGE {
            println!("Sun sensor powered on!");
        } else if self.voltage >= POWER_ON_VOLTAGE && value < POWER_ON_VOLTAGE {
            println!("Sun sensor powered off!");
        }
        self.voltage = value;
    }

    pub fn new() -> Self {
        Self { voltage: 0.0 }
    }
}
