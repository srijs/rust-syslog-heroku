use std::string::String;
use std::str::FromStr;

use chrono::DateTime;
use chrono::offset::FixedOffset;

use severity::Severity;
use parser::{ParseError, parse_message};

/// Represents a message being send to a Heroku Log Drain
#[derive(Clone, Debug)]
pub struct Message {
    pub severity: Severity,
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub hostname: Option<String>,
    pub appname: Option<String>,
    pub procid: Option<String>,
    pub msgid: Option<String>,
    pub msg: String,
}

impl FromStr for Message {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_message(s)
    }
}
