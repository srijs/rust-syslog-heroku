#[derive(Copy, Clone, Debug, PartialEq)]
/// Syslog Severities from RFC 5424.
pub enum Severity {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

impl Severity {
    /// Convert an int (as used in the wire serialization) into a `Severity`
    ///
    /// Returns an Option, but the wire protocol will only include 0..7, so should
    /// never return None in practical usage.
    pub(crate) fn from_int(i: u32) -> Option<Self> {
        match i {
            0 => Some(Severity::Emergency),
            1 => Some(Severity::Alert),
            2 => Some(Severity::Critical),
            3 => Some(Severity::Error),
            4 => Some(Severity::Warning),
            5 => Some(Severity::Notice),
            6 => Some(Severity::Info),
            7 => Some(Severity::Debug),
            _ => None,
        }
    }
}
