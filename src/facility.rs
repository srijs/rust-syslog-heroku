/// Syslog facilities. Taken From RFC 5424, but I've heard that some platforms mix these around.
/// Names are from Linux.
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Facility {
    LOG_KERN = 0,
    LOG_USER = 1,
    LOG_MAIL = 2,
    LOG_DAEMON = 3,
    LOG_AUTH = 4,
    LOG_SYSLOG = 5,
    LOG_LPR = 6,
    LOG_NEWS = 7,
    LOG_UUCP = 8,
    LOG_CRON = 9,
    LOG_AUTHPRIV = 10,
    LOG_FTP = 11,
    LOG_NTP = 12,
    LOG_AUDIT = 13,
    LOG_ALERT = 14,
    LOG_CLOCKD = 15,
    LOG_LOCAL0 = 16,
    LOG_LOCAL1 = 17,
    LOG_LOCAL2 = 18,
    LOG_LOCAL3 = 19,
    LOG_LOCAL4 = 20,
    LOG_LOCAL5 = 21,
    LOG_LOCAL6 = 22,
    LOG_LOCAL7 = 23,
}

impl Facility {
    /// Convert an int (as used in the wire serialization) into a `Facility`
    pub fn from_int(i: u32) -> Option<Self> {
        match i {
            0 => Some(Facility::LOG_KERN),
            1 => Some(Facility::LOG_USER),
            2 => Some(Facility::LOG_MAIL),
            3 => Some(Facility::LOG_DAEMON),
            4 => Some(Facility::LOG_AUTH),
            5 => Some(Facility::LOG_SYSLOG),
            6 => Some(Facility::LOG_LPR),
            7 => Some(Facility::LOG_NEWS),
            8 => Some(Facility::LOG_UUCP),
            9 => Some(Facility::LOG_CRON),
            10 => Some(Facility::LOG_AUTHPRIV),
            11 => Some(Facility::LOG_FTP),
            12 => Some(Facility::LOG_NTP),
            13 => Some(Facility::LOG_AUDIT),
            14 => Some(Facility::LOG_ALERT),
            15 => Some(Facility::LOG_CLOCKD),
            16 => Some(Facility::LOG_LOCAL0),
            17 => Some(Facility::LOG_LOCAL1),
            18 => Some(Facility::LOG_LOCAL2),
            19 => Some(Facility::LOG_LOCAL3),
            20 => Some(Facility::LOG_LOCAL4),
            21 => Some(Facility::LOG_LOCAL5),
            22 => Some(Facility::LOG_LOCAL6),
            23 => Some(Facility::LOG_LOCAL7),
            _ => None,
        }
    }

    /// Convert a syslog facility into a unique string representation
    pub fn as_str(&self) -> &'static str {
        match *self {
            Facility::LOG_KERN => "kern",
            Facility::LOG_USER => "user",
            Facility::LOG_MAIL => "mail",
            Facility::LOG_DAEMON => "daemon",
            Facility::LOG_AUTH => "auth",
            Facility::LOG_SYSLOG => "syslog",
            Facility::LOG_LPR => "lpr",
            Facility::LOG_NEWS => "news",
            Facility::LOG_UUCP => "uucp",
            Facility::LOG_CRON => "cron",
            Facility::LOG_AUTHPRIV => "authpriv",
            Facility::LOG_FTP => "ftp",
            Facility::LOG_NTP => "ntp",
            Facility::LOG_AUDIT => "audit",
            Facility::LOG_ALERT => "alert",
            Facility::LOG_CLOCKD => "clockd",
            Facility::LOG_LOCAL0 => "local0",
            Facility::LOG_LOCAL1 => "local1",
            Facility::LOG_LOCAL2 => "local2",
            Facility::LOG_LOCAL3 => "local3",
            Facility::LOG_LOCAL4 => "local4",
            Facility::LOG_LOCAL5 => "local5",
            Facility::LOG_LOCAL6 => "local6",
            Facility::LOG_LOCAL7 => "local7",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Facility;

    #[test]
    fn test_deref() {
        assert_eq!(Facility::LOG_KERN.as_str(), "kern");
    }
}
