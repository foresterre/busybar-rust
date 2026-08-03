use std::fmt;

use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::types::output_format::OutputFormatArg;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnsupportedEvent {
    operation: &'static str,
    reason: &'static str,
    output_format: &'static str,
}

impl UnsupportedEvent {
    pub fn output_format(
        operation: &'static str,
        reason: &'static str,
        format: OutputFormatArg,
    ) -> Option<Self> {
        let output_format = match format {
            OutputFormatArg::Text => return None,
            OutputFormatArg::Json => "json",
        };

        Some(Self {
            operation,
            reason,
            output_format,
        })
    }
}

impl From<UnsupportedEvent> for CliEvent {
    fn from(event: UnsupportedEvent) -> Self {
        CliEvent::Unsupported(event)
    }
}

impl fmt::Display for UnsupportedEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} is not supported with the {} output format, {}",
            self.operation, self.output_format, self.reason
        )
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn an_operation_is_supported_by_the_text_output_format() {
        assert_eq!(
            UnsupportedEvent::output_format("mirror", "it paints frames", OutputFormatArg::Text),
            None
        );
    }

    #[test]
    fn reports_the_output_format_an_operation_cannot_serve() {
        let event =
            UnsupportedEvent::output_format("mirror", "it paints frames", OutputFormatArg::Json)
                .unwrap();

        assert_eq!(
            event.to_string(),
            "mirror is not supported with the json output format, it paints frames"
        );
        assert_eq!(
            serde_json::to_value(CliEvent::from(event)).unwrap(),
            json!({
                "event": "unsupported",
                "operation": "mirror",
                "reason": "it paints frames",
                "output_format": "json",
            })
        );
    }
}
