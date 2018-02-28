//! In-memory representation of a single Syslog message.

use std::string::String;
use std::str::FromStr;

use chrono::DateTime;
use chrono::offset::FixedOffset;

use severity::Severity;
use parser::{ParseError, parse_message};

#[derive(Clone, Debug, PartialEq, Eq)]
/// `ProcID`s are usually numeric PIDs; however, on some systems, they may be something else
pub enum ProcId {
    Pid(u32),
    Name(String)
}

#[derive(Clone,Debug)]
pub struct Message {
    pub severity: Severity,
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub hostname: Option<String>,
    pub appname: Option<String>,
    pub procid: Option<ProcId>,
    pub msgid: Option<String>,
    pub msg: String,
}

impl FromStr for Message {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_message(s)
    }
}
