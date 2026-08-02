use std::sync::Arc;

use storyteller::EventHandler;

use crate::reporter::events::CliEvent;
use crate::reporter::output::Output;

pub struct HumanHandler {
    output: Arc<Output>,
}

impl HumanHandler {
    pub fn new(output: Arc<Output>) -> Self {
        Self { output }
    }
}

impl EventHandler for HumanHandler {
    type Event = CliEvent;

    fn handle(&self, event: Self::Event) {
        self.output.write_line(&event.to_string());
    }

    fn finish(&self) {
        self.output.flush();
    }
}
