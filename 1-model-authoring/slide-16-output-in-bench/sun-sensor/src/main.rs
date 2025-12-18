use nexosim::ports::EventSlot;
use nexosim::simulation::{Mailbox, SimInit};
use nexosim::time::MonotonicTime;

use sun_sensor::{SampleCommand, SunSensor};

fn main() {
    let mut sensor = SunSensor::new(0);
    let sensor_mbox = Mailbox::new();
    let sensor_addr = sensor_mbox.address();

    let mut sink = EventSlot::new();
    sensor.tm.connect_sink(&sink);

    let mut sim = SimInit::new()
        .add_model(sensor, sensor_mbox, "SUN_SENSOR")
        .init(MonotonicTime::EPOCH)
        .unwrap();

    // Power on.
    sim.process_event(SunSensor::voltage_in, 5.0, &sensor_addr)
        .unwrap();

    // Send a sample command.
    let cmd = SampleCommand {
        src_address: 1,
        dest_address: 0,
    };
    sim.process_event(SunSensor::tc, cmd, &sensor_addr).unwrap();

    let reply = sink.next().unwrap();
    println!("{:?}", reply)
}
