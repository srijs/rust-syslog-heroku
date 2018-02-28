//! In-memory representation of a single Syslog message.

use std::string::String;

use chrono::DateTime;
use chrono::offset::FixedOffset;

use severity::Severity;
use facility::Facility;

#[derive(Clone, Debug, PartialEq, Eq)]
/// `ProcID`s are usually numeric PIDs; however, on some systems, they may be something else
pub enum ProcId {
    Pid(u32),
    Name(String)
}

#[derive(Clone,Debug)]
pub struct Message {
    pub severity: Severity,
    pub facility: Facility,
    pub version: u32,
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub hostname: Option<String>,
    pub appname: Option<String>,
    pub procid: Option<ProcId>,
    pub msgid: Option<String>,
    pub msg: String,
}
