//! In-memory representation of a single Syslog message.

use std::string::String;

use chrono::DateTime;
use chrono::offset::FixedOffset;

use severity;
use facility;

#[derive(Clone,Debug,PartialEq,Eq)]
/// `ProcID`s are usually numeric PIDs; however, on some systems, they may be something else
pub enum ProcIdType {
    PID(u32),
    Name(String)
}

#[derive(Clone,Debug)]
pub struct SyslogMessage {
    pub severity: severity::SyslogSeverity,
    pub facility: facility::SyslogFacility,
    pub version: u32,
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub hostname: Option<String>,
    pub appname: Option<String>,
    pub procid: Option<ProcIdType>,
    pub msgid: Option<String>,
    pub msg: String,
}
