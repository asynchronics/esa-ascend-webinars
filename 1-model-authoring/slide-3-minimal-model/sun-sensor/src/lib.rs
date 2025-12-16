use nexosim::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SunSensor;

#[Model]
impl SunSensor {}
