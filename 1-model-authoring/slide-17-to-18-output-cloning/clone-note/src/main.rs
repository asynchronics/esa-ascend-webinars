use nexosim::Model;
use nexosim::ports::Output;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Inner {
    out: Output<u8>,
}

impl Inner {
    async fn broadcast(&mut self, value: u8) {
        self.out.send(value).await;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Outer {
    pub out: Output<u8>,
    inner: Inner,
}

#[Model]
impl Outer {
    pub async fn input(&mut self) {
        self.inner.broadcast(42).await;
    }

    pub fn new() -> Self {
        let out = Output::default();
        let inner = Inner { out: out.clone() };
        Self { out, inner }
    }
}

use nexosim::ports::EventSlot;
use nexosim::simulation::{Mailbox, SimInit};
use nexosim::time::MonotonicTime;

fn main() {
    let mut model = Outer::new();
    let mailbox = Mailbox::new();
    let address = mailbox.address();

    let mut sink = EventSlot::new();
    model.out.connect_sink(&sink);

    let mut sim = SimInit::new()
        .add_model(model, mailbox, "MODEL")
        .init(MonotonicTime::EPOCH)
        .unwrap();

    sim.process_event(Outer::input, (), address).unwrap();

    let reply = sink.next().unwrap();
    println!("{reply}");
}
