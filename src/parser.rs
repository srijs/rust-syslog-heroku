use std::str::FromStr;
use std::str;
use std::num;
use std::string;

use time;


use severity;
use facility;
use message::{time_t,SyslogMessage,ProcIdType,StructuredData};

#[derive(Debug)]
pub enum ParseErr {
    RegexDoesNotMatchErr,
    BadSeverityInPri,
    BadFacilityInPri,
    UnexpectedEndOfInput,
    TooFewDigits,
    TooManyDigits,
    InvalidUTCOffset,
    BaseUnicodeError(str::Utf8Error),
    UnicodeError(string::FromUtf8Error),
    ExpectedTokenErr(char),
    IntConversionErr(num::ParseIntError),
    MissingField(&'static str)
}

// We parse with this super-duper-dinky hand-coded recursive descent parser because we don't really
// have much other choice:
//
//  - Regexp is much slower (at least a factor of 4), and we still end up having to parse the
//    somewhat-irregular SD
//  - LALRPOP requires non-ambiguous tokenization
//  - Rust-PEG doesn't work on anything except nightly
//
// So here we are. The macros make it a bit better.
//
// General convention is that the parse state is represented by a string slice named "rest"; the
// macros will update that slice as they consume tokens.

macro_rules! maybe_expect_char {
    ($s:expr, $e: expr) => (match $s.chars().next() {
        Some($e) => Some(&$s[1..]),
        _ => None,
    })
}

macro_rules! take_item {
    ($e:expr, $r:expr) => {{
        let (t, r) = $e?;
        $r = r;
        t
    }}
}


type ParseResult<T> = Result<T, ParseErr>;

macro_rules! take_char {
    ($e: expr, $c:expr) => {{
        $e = match $e.chars().next() {
            Some($c) => &$e[1..],
            Some(_) => {
                //println!("Error with rest={:?}", $e);
                return Err(ParseErr::ExpectedTokenErr($c));
            },
            None => {
                //println!("Error with rest={:?}", $e);
                return Err(ParseErr::UnexpectedEndOfInput);
            }
        }
    }}
}

fn take_while<F>(input: &str, f: F, max_chars: usize) -> (&str, Option<&str>)
    where F: Fn(char) -> bool {

    for (idx, chr) in input.char_indices() {
        if !f(chr) {
            return (&input[..idx], Some(&input[idx..]));
        }
        if idx == max_chars {
            return (&input[..idx], Some(&input[idx..]));
        }
    }
    ("", None)
}

fn parse_pri_val(pri: i32) -> ParseResult<(severity::SyslogSeverity, facility::SyslogFacility)> {
    let sev = severity::SyslogSeverity::from_int(pri & 0x7).ok_or(ParseErr::BadSeverityInPri)?;
    let fac = facility::SyslogFacility::from_int(pri >> 3).ok_or(ParseErr::BadFacilityInPri)?;
    Ok((sev, fac))
}

fn parse_num(s: &str, min_digits: usize, max_digits: usize) -> ParseResult<(i32, &str)> {
    let (res, rest1) = take_while(s, |c| c >= '0' && c <= '9', max_digits);
    let rest = rest1.ok_or(ParseErr::UnexpectedEndOfInput)?;
    if res.len() < min_digits {
        Err(ParseErr::TooFewDigits)
    } else if res.len() > max_digits {
        Err(ParseErr::TooManyDigits)
    } else {
        Ok((i32::from_str(res).map_err(ParseErr::IntConversionErr)?, rest))
    }
}

fn parse_timestamp(m: &str) -> ParseResult<(Option<time_t>, &str)> {
    let mut rest = m;
    if rest.starts_with('-') {
        return Ok((None, &rest[1..]))
    }
    let mut tm = time::empty_tm();
    tm.tm_year = take_item!(parse_num(rest, 4, 4), rest) - 1900;
    take_char!(rest, '-');
    tm.tm_mon = take_item!(parse_num(rest, 2, 2), rest) - 1;
    take_char!(rest, '-');
    tm.tm_mday = take_item!(parse_num(rest, 2, 2), rest);
    take_char!(rest, 'T');
    tm.tm_hour = take_item!(parse_num(rest, 2, 2), rest);
    take_char!(rest, ':');
    tm.tm_min = take_item!(parse_num(rest, 2, 2), rest);
    take_char!(rest, ':');
    tm.tm_sec = take_item!(parse_num(rest, 2, 2), rest);
    if rest.starts_with('.') {
        take_char!(rest, '.');
        take_item!(parse_num(rest, 1, 6), rest);
    }
    // Tm::utcoff is totally broken, don't use it.
    let utc_offset_mins = match rest.chars().next() {
        None => 0,
        Some('Z') => {
            rest = &rest[1..];
            0
        },
        Some(c) => {
            let (sign, irest) = match c {
                '+' => (1, &rest[1..]),
                '-' => (-1, &rest[1..]),
                _ => { return Err(ParseErr::InvalidUTCOffset); }
            };
            let hours = i32::from_str(&irest[0..2]).map_err(ParseErr::IntConversionErr)?;
            let minutes = i32::from_str(&irest[3..5]).map_err(ParseErr::IntConversionErr)?;
            rest = &irest[5..];
            minutes + hours * 60 * sign
        }
    };
    tm = tm + time::Duration::minutes(i64::from(utc_offset_mins));
    tm.tm_isdst = -1;
    Ok((Some(tm.to_utc().to_timespec().sec), rest))
}

fn parse_term(m: &str, min_length: usize, max_length: usize) -> ParseResult<(Option<String>, &str)> {
    if m.starts_with('-') {
        return Ok((None, &m[1..]))
    }
    let byte_ary = m.as_bytes();
    for (idx, chr) in byte_ary.iter().enumerate() {
        //println!("idx={:?}, buf={:?}, chr={:?}", idx, buf, chr);
        if *chr < 33 || *chr > 126 {
            if idx < min_length {
                return Err(ParseErr::TooFewDigits);
            }
            let utf8_ary = str::from_utf8(&byte_ary[..idx]).map_err(ParseErr::BaseUnicodeError)?;
            return Ok((Some(String::from(utf8_ary)), &m[idx..]));
        }
        if idx >= max_length {
            let utf8_ary = str::from_utf8(&byte_ary[..idx]).map_err(ParseErr::BaseUnicodeError)?;
            return Ok((Some(String::from(utf8_ary)), &m[idx..]));
        }
    }
    Err(ParseErr::UnexpectedEndOfInput)
}


fn parse_message_s(m: &str) -> ParseResult<SyslogMessage> {
    let mut rest = m;
    take_char!(rest, '<');
    let prival = take_item!(parse_num(rest, 1, 3), rest);
    take_char!(rest, '>');
    let (sev, fac) = parse_pri_val(prival)?;
    let version = take_item!(parse_num(rest, 1, 2), rest);
    //println!("got version {:?}, rest={:?}", version, rest);
    take_char!(rest, ' ');
    let timestamp = take_item!(parse_timestamp(rest), rest);
    //println!("got timestamp {:?}, rest={:?}", timestamp, rest);
    take_char!(rest, ' ');
    let hostname = take_item!(parse_term(rest, 1, 255), rest);
    //println!("got hostname {:?}, rest={:?}", hostname, rest);
    take_char!(rest, ' ');
    let appname = take_item!(parse_term(rest, 1, 48), rest);
    //println!("got appname {:?}, rest={:?}", appname, rest);
    take_char!(rest, ' ');
    let procid_r = take_item!(parse_term(rest, 1, 128), rest);
    let procid = match procid_r {
        None => None,
        Some(s) => Some(match i32::from_str(&s) {
            Ok(n) => ProcIdType::PID(n),
            Err(_) => ProcIdType::Name(s)
        })
    };
    //println!("got procid {:?}, rest={:?}", procid, rest);
    take_char!(rest, ' ');
    let msgid = take_item!(parse_term(rest, 1, 32), rest);
    //println!("got sd {:?}, rest={:?}", sd, rest);
    rest = match maybe_expect_char!(rest, ' ') {
        Some(r) => r,
        None => rest
    };
    let msg = String::from(rest);

    Ok(SyslogMessage {
        severity: sev,
        facility: fac,
        version: version,
        timestamp: timestamp,
        hostname: hostname,
        appname: appname,
        procid: procid,
        msgid: msgid,
        sd: StructuredData::new_empty(),
        msg: msg
    })
}



/// Parse a string into a `SyslogMessage` object
///
/// # Arguments
///
///  * `s`: Anything convertible to a string
///
/// # Returns
///
///  * `ParseErr` if the string is not parseable as an RFC5424 message
///
/// # Example
///
/// ```
/// use syslog_rfc5424::parse_message;
///
/// let message = parse_message("<78>1 2016-01-15T00:04:01+00:00 host1 CROND 10391 - [meta sequenceId=\"29\"] some_message").unwrap();
///
/// assert!(message.hostname.unwrap() == "host1");
/// ```
pub fn parse_message<S: AsRef<str>> (s: S) -> ParseResult<SyslogMessage> {
    parse_message_s(s.as_ref())
}


#[cfg(test)]
mod tests {
    use super::parse_message;

    use message::ProcIdType;
    use facility::SyslogFacility;
    use severity::SyslogSeverity;

    #[test]
    fn test_router_message() {
        let msg = parse_message(r#"<158>1 2014-08-04T18:28:43.078581+00:00 host heroku router - at=info method=GET path="/foo" host=app-name-7277.herokuapp.com request_id=e5bb3580-44b0-46d2-aad3-185263641044 fwd="50.168.96.221" dyno=web.1 connect=0ms service=2ms status=200 bytes=415"#)
            .expect("Should parse router message");
        assert_eq!(msg.facility, SyslogFacility::LOG_LOCAL3);
        assert_eq!(msg.severity, SyslogSeverity::SEV_INFO);
        assert_eq!(msg.timestamp, Some(1407176923));
        assert_eq!(msg.hostname, Some("host".to_owned()));
        assert_eq!(msg.appname, Some("heroku".to_owned()));
        assert_eq!(msg.procid, Some(ProcIdType::Name("router".to_owned())));
        assert_eq!(msg.msgid, None);
    }

    #[test]
    fn test_web_app_message() {
        let msg = parse_message(r#"<190>1 2014-08-04T18:28:43.015630+00:00 host app web.1 - 50.168.96.221 - - [04/Aug/2014 18:28:43] "GET /foo HTTP/1.1" 200 12 0.0019"#)
            .expect("Should parse web app message");
        assert_eq!(msg.facility, SyslogFacility::LOG_LOCAL7);
        assert_eq!(msg.severity, SyslogSeverity::SEV_INFO);
        assert_eq!(msg.timestamp, Some(1407176923));
        assert_eq!(msg.hostname, Some("host".to_owned()));
        assert_eq!(msg.appname, Some("app".to_owned()));
        assert_eq!(msg.procid, Some(ProcIdType::Name("web.1".to_owned())));
        assert_eq!(msg.msgid, None);
    }
}
