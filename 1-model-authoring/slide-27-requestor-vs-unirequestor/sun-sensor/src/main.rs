use nexosim::ports::EventSlot;
use nexosim::simulation::{Mailbox, SimInit};
use nexosim::time::MonotonicTime;

use sun_sensor::{Dynamics, SampleCommand, SunSensor};

fn main() {
    let dynamics = Dynamics;
    let dynamics_mbox = Mailbox::new();
    let dynamics_addr = dynamics_mbox.address();

    let mut sensor = SunSensor::new(0);
    let sensor_mbox = Mailbox::new();
    let sensor_addr = sensor_mbox.address();

    sensor
        .sun_position_req
        .connect(Dynamics::sun_position, dynamics_addr);

    let mut sink = EventSlot::new();
    sensor.tm.connect_sink(&sink);

    let mut sim = SimInit::new()
        .add_model(sensor, sensor_mbox, "SUN_SENSOR")
        .add_model(dynamics, dynamics_mbox, "DYNAMICS")
        .init(MonotonicTime::EPOCH)
        .unwrap();

    // Power on.
    sim.process_event(SunSensor::voltage_in, 5.0, &sensor_addr)
        .unwrap();

    let cmd = SampleCommand {
        src_address: 1,
        dest_address: 0,
    };

    // Advance simulation time by ten seconds.
    sim.step_until(MonotonicTime::new(10, 0).unwrap()).unwrap();

    // Send a sample command.
    sim.process_event(SunSensor::tc, cmd.clone(), &sensor_addr)
        .unwrap();

    let reply = sink.next().unwrap();
    println!("{:?}", reply);

    // Advance simulation time again.
    sim.step_until(MonotonicTime::new(20, 0).unwrap()).unwrap();

    // Send another sample command.
    sim.process_event(SunSensor::tc, cmd, &sensor_addr).unwrap();

    let reply = sink.next().unwrap();
    println!("{:?}", reply);
}
