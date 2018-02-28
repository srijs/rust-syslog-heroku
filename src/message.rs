//! In-memory representation of a single Syslog message.

use std::string::String;

#[allow(non_camel_case_types)]
pub type time_t = i64;
#[allow(non_camel_case_types)]
pub type pid_t = i32;
#[allow(non_camel_case_types)]
pub type msgid_t = String;
pub type MessageType = String;

use severity;
use facility;


#[derive(Clone,Debug,PartialEq,Eq)]
/// `ProcID`s are usually numeric PIDs; however, on some systems, they may be something else
pub enum ProcIdType {
    PID(pid_t),
    Name(String)
}

#[derive(Clone,Debug)]
pub struct SyslogMessage {
    pub severity: severity::SyslogSeverity,
    pub facility: facility::SyslogFacility,
    pub version: i32,
    pub timestamp: Option<time_t>,
    pub hostname: Option<String>,
    pub appname: Option<String>,
    pub procid: Option<ProcIdType>,
    pub msgid: Option<msgid_t>,
    pub msg: String,
}
