use nexosim::ports::{EventSlot, UniRequestor};
use nexosim::simulation::{Mailbox, SimInit};
use nexosim::time::MonotonicTime;

use sun_sensor::{Dynamics, Obc, SunSensor};

const OBC_ADDR: u8 = 0;
const SENSOR_ADDR: u8 = 1;

fn main() {
    let dynamics = Dynamics;
    let dynamics_mbox = Mailbox::new();
    let dynamics_addr = dynamics_mbox.address();

    let requestor = UniRequestor::new(Dynamics::sun_position, &dynamics_addr);

    let mut sensor = SunSensor::new(SENSOR_ADDR, requestor);
    let sensor_mbox = Mailbox::new();
    let sensor_addr = sensor_mbox.address();

    let mut obc = Obc::new(OBC_ADDR, SENSOR_ADDR);
    let obc_mbox = Mailbox::new();

    obc.tm.connect(SunSensor::tc, &sensor_addr);

    let mut sink = EventSlot::new();
    sensor.tm.connect_sink(&sink);

    let mut sim = SimInit::new()
        .add_model(sensor, sensor_mbox, "SUN_SENSOR")
        .add_model(dynamics, dynamics_mbox, "DYNAMICS")
        .add_model(obc, obc_mbox, "OBC")
        .init(MonotonicTime::EPOCH)
        .unwrap();

    // Power on.
    sim.process_event(SunSensor::voltage_in, 5.0, &sensor_addr)
        .unwrap();

    // Advance simulation time by ten seconds.
    sim.step_until(MonotonicTime::new(10, 0).unwrap()).unwrap();

    let reply = sink.next().unwrap();
    println!("{:?}", reply);

    // Advance simulation time again.
    sim.step_until(MonotonicTime::new(20, 0).unwrap()).unwrap();

    let reply = sink.next().unwrap();
    println!("{:?}", reply);
}
