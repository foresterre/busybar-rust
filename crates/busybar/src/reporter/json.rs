use std::sync::Arc;

use storyteller::EventHandler;

use crate::reporter::events::CliEvent;
use crate::reporter::output::Output;

pub struct JsonHandler {
    output: Arc<Output>,
}

impl JsonHandler {
    pub fn new(output: Arc<Output>) -> Self {
        Self { output }
    }
}

impl EventHandler for JsonHandler {
    type Event = CliEvent;

    fn handle(&self, event: Self::Event) {
        match serde_json::to_string(&event) {
            Ok(line) => self.output.write_line(&line),
            Err(error) => self.output.record_error(error.into()),
        }
    }

    fn finish(&self) {
        self.output.flush();
    }
}
