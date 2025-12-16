use nexosim::Model;
use serde::{Deserialize, Serialize};

const POWER_ON_VOLTAGE: f32 = 5.0;

#[derive(Serialize, Deserialize)]
pub struct SunSensor {
    voltage: f32,
    address: u8,
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

    pub fn tc(&mut self, cmd: SampleCommand) {
        if self.voltage < POWER_ON_VOLTAGE || self.address != cmd.dest_address {
            return;
        }

        let sun_position = [0.0, 1.0, 10.0];
        println!("{:?}", cartesian_to_spherical(sun_position));
    }

    pub fn new(address: u8) -> Self {
        Self {
            address,
            voltage: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct SampleCommand {
    pub src_address: u8,
    pub dest_address: u8,
}

/// Converts a 3-D coordinate in a cartesian system [x, y, z] to a coordinate in a
/// spherical system [r, θ, ϕ].
fn cartesian_to_spherical(arr: [f32; 3]) -> [f32; 3] {
    let radius = f32::sqrt(arr.iter().map(|x| x.powi(2)).sum());
    if radius == 0.0 {
        return [0.0, 0.0, 0.0];
    }

    let polar = (arr[2] / radius).acos();
    if polar == 0.0 {
        return [radius, 0.0, 0.0];
    }

    let azimuthal =
        arr[1].signum() * (arr[0] / f32::sqrt(arr[0..2].iter().map(|x| x.powi(2)).sum())).acos();

    [radius, polar, azimuthal]
}
