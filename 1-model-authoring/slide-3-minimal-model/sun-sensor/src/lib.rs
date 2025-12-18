use nexosim::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SunSensor;

#[Model]
impl SunSensor {}
