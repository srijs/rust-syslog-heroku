#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// Syslog Severities from RFC 5424.
pub enum Severity {
    SEV_EMERG = 0,
    SEV_ALERT = 1,
    SEV_CRIT = 2,
    SEV_ERR = 3,
    SEV_WARNING = 4,
    SEV_NOTICE = 5,
    SEV_INFO = 6,
    SEV_DEBUG = 7,
}

impl Severity {
    /// Convert an int (as used in the wire serialization) into a `Severity`
    ///
    /// Returns an Option, but the wire protocol will only include 0..7, so should
    /// never return None in practical usage.
    pub fn from_int(i: u32) -> Option<Self> {
        match i {
            0 => Some(Severity::SEV_EMERG),
            1 => Some(Severity::SEV_ALERT),
            2 => Some(Severity::SEV_CRIT),
            3 => Some(Severity::SEV_ERR),
            4 => Some(Severity::SEV_WARNING),
            5 => Some(Severity::SEV_NOTICE),
            6 => Some(Severity::SEV_INFO),
            7 => Some(Severity::SEV_DEBUG),
            _ => None,
        }
    }

    /// Convert a syslog severity into a unique string representation
    pub fn as_str(&self) -> &'static str {
        match *self {
            Severity::SEV_EMERG => "emerg",
            Severity::SEV_ALERT => "alert",
            Severity::SEV_CRIT => "crit",
            Severity::SEV_ERR => "err",
            Severity::SEV_WARNING => "warning",
            Severity::SEV_NOTICE => "notice",
            Severity::SEV_INFO => "info",
            Severity::SEV_DEBUG => "debug"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Severity;

    #[test]
    fn test_deref() {
        assert_eq!(Severity::SEV_EMERG.as_str(), "emerg");
        assert_eq!(Severity::SEV_ALERT.as_str(), "alert");
        assert_eq!(Severity::SEV_CRIT.as_str(), "crit");
        assert_eq!(Severity::SEV_ERR.as_str(), "err");
        assert_eq!(Severity::SEV_WARNING.as_str(), "warning");
        assert_eq!(Severity::SEV_NOTICE.as_str(), "notice");
        assert_eq!(Severity::SEV_INFO.as_str(), "info");
        assert_eq!(Severity::SEV_DEBUG.as_str(), "debug");
    }
}
