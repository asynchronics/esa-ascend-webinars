use std::time::Duration;

use nexosim::model::{Context, InitializedModel, Model, ProtoModel, schedulable};
use nexosim::ports::{Output, UniRequestor};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rand_distr::StandardNormal;
use serde::{Deserialize, Serialize};

const POWER_ON_VOLTAGE: f32 = 5.0;

pub struct SunSensorEnv {
    rng: SmallRng,
    distr: StandardNormal,
}

pub struct ProtoSunSensor {
    pub tm: Output<SampleReply>,
    pub sun_position_req: UniRequestor<(), [f32; 3]>,
    address: u8,
}

impl ProtoSunSensor {
    pub fn new(address: u8, sun_position_req: UniRequestor<(), [f32; 3]>) -> Self {
        Self {
            address,
            tm: Output::default(),
            sun_position_req,
        }
    }
}

impl ProtoModel for ProtoSunSensor {
    type Model = SunSensor;

    fn build(self, _: &mut nexosim::model::BuildContext<Self>) -> (SunSensor, SunSensorEnv) {
        (
            SunSensor::new(self.address, self.sun_position_req, self.tm),
            SunSensorEnv {
                rng: SmallRng::from_entropy(),
                distr: StandardNormal,
            },
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct SunSensor {
    voltage: f32,
    address: u8,
    pub tm: Output<SampleReply>,
    pub sun_position_req: UniRequestor<(), [f32; 3]>,
}

#[Model(type Env = SunSensorEnv)]
impl SunSensor {
    pub fn voltage_in(&mut self, value: f32) {
        if self.voltage < POWER_ON_VOLTAGE && value >= POWER_ON_VOLTAGE {
            println!("Sun sensor powered on!");
        } else if self.voltage >= POWER_ON_VOLTAGE && value < POWER_ON_VOLTAGE {
            println!("Sun sensor powered off!");
        }
        self.voltage = value;
    }

    pub async fn tc(&mut self, cmd: SampleCommand, _: &Context<Self>, env: &mut SunSensorEnv) {
        if self.voltage < POWER_ON_VOLTAGE || self.address != cmd.dest_address {
            return;
        }

        let sun_position = self.sun_position_req.send(()).await;
        let [_, mut polar_angle, azimuthal_angle] = cartesian_to_spherical(sun_position);

        polar_angle += env.rng.sample::<f32, StandardNormal>(env.distr) * 0.1;

        let reply = SampleReply {
            src_address: cmd.dest_address,
            dest_address: cmd.src_address,
            polar_angle,
            azimuthal_angle,
        };
        self.tm.send(reply).await;
    }

    pub fn new(
        address: u8,
        sun_position_req: UniRequestor<(), [f32; 3]>,
        tm: Output<SampleReply>,
    ) -> Self {
        Self {
            address,
            voltage: 0.0,
            tm,
            sun_position_req,
        }
    }
}

#[derive(Clone)]
pub struct SampleCommand {
    pub src_address: u8,
    pub dest_address: u8,
}

#[derive(Clone, Debug)]
pub struct SampleReply {
    pub src_address: u8,
    pub dest_address: u8,
    pub polar_angle: f32,
    pub azimuthal_angle: f32,
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

#[derive(Serialize, Deserialize)]
pub struct Dynamics;

#[Model]
impl Dynamics {
    pub async fn sun_position(&mut self, _: (), ctx: &Context<Self>) -> [f32; 3] {
        let t = ctx.time().as_secs() as f32 + ctx.time().subsec_nanos() as f32 * 1e-9;

        // Movement in an arbitrary straight line
        [2.3 + t * 5.7, 1.1 * t, 1.3 * t]
    }
}

#[derive(Serialize, Deserialize)]
pub struct Obc {
    pub tm: Output<SampleCommand>,
    address: u8,
    sensor_address: u8,
}

#[Model]
impl Obc {
    pub fn new(address: u8, sensor_address: u8) -> Self {
        Self {
            tm: Output::default(),
            address,
            sensor_address,
        }
    }

    #[nexosim(schedulable)]
    async fn send_sample_command(&mut self) {
        self.tm
            .send(SampleCommand {
                src_address: self.address,
                dest_address: self.sensor_address,
            })
            .await;
    }

    #[nexosim(init)]
    async fn init(self, ctx: &Context<Self>) -> InitializedModel<Self> {
        ctx.schedule_periodic_event(
            Duration::new(1, 0),
            Duration::new(1, 0),
            schedulable!(Self::send_sample_command),
            (),
        )
        .unwrap();

        self.into()
    }
}
