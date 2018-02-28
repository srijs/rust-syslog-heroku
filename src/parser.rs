use std::str::FromStr;
use std::str;
use std::num;
use std::string;

use chrono::{DateTime, FixedOffset, TimeZone};

use severity::Severity;
use message::{Message, ProcId};

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "invalid severity value in priority header")]
    BadSeverityInPri,
    #[fail(display = "unexpected end of input")]
    UnexpectedEndOfInput,
    #[fail(display = "too few digits")]
    TooFewDigits,
    #[fail(display = "too many digits")]
    TooManyDigits,
    #[fail(display = "invalid date time")]
    InvalidDateTime,
    #[fail(display = "unsupported version {}", _0)]
    UnsupportedVersion(u32),
    #[fail(display = "unicode error: {}", _0)]
    BaseUnicodeError(#[cause] str::Utf8Error),
    #[fail(display = "unicode error: {}", _0)]
    UnicodeError(#[cause] string::FromUtf8Error),
    #[fail(display = "expected token '{}'", _0)]
    ExpectedTokenErr(char),
    #[fail(display = "parse int error: {}", _0)]
    IntConversionErr(#[cause] num::ParseIntError),
    #[fail(display = "missing field '{}'", _0)]
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

type ParseResult<T> = Result<T, ParseError>;

macro_rules! take_char {
    ($e: expr, $c:expr) => {{
        $e = match $e.chars().next() {
            Some($c) => &$e[1..],
            Some(_) => {
                //println!("Error with rest={:?}", $e);
                return Err(ParseError::ExpectedTokenErr($c));
            },
            None => {
                //println!("Error with rest={:?}", $e);
                return Err(ParseError::UnexpectedEndOfInput);
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

fn parse_pri_val(pri: u32) -> ParseResult<Severity> {
    Severity::from_int(pri & 0x7).ok_or(ParseError::BadSeverityInPri)
}

fn parse_num(s: &str, min_digits: usize, max_digits: usize) -> ParseResult<(u32, &str)> {
    let (res, rest1) = take_while(s, |c| c >= '0' && c <= '9', max_digits);
    let rest = rest1.ok_or(ParseError::UnexpectedEndOfInput)?;
    if res.len() < min_digits {
        Err(ParseError::TooFewDigits)
    } else if res.len() > max_digits {
        Err(ParseError::TooManyDigits)
    } else {
        Ok((u32::from_str(res).map_err(ParseError::IntConversionErr)?, rest))
    }
}

fn parse_timestamp(m: &str) -> ParseResult<(Option<DateTime<FixedOffset>>, &str)> {
    let mut rest = m;
    if rest.starts_with('-') {
        return Ok((None, &rest[1..]))
    }
    let tm_year = take_item!(parse_num(rest, 4, 4), rest);
    take_char!(rest, '-');
    let tm_mon = take_item!(parse_num(rest, 2, 2), rest);
    take_char!(rest, '-');
    let tm_mday = take_item!(parse_num(rest, 2, 2), rest);
    take_char!(rest, 'T');
    let tm_hour = take_item!(parse_num(rest, 2, 2), rest);
    take_char!(rest, ':');
    let tm_min = take_item!(parse_num(rest, 2, 2), rest);
    take_char!(rest, ':');
    let tm_sec = take_item!(parse_num(rest, 2, 2), rest);
    if rest.starts_with('.') {
        take_char!(rest, '.');
        take_item!(parse_num(rest, 1, 6), rest);
    }
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
                _ => { return Err(ParseError::InvalidDateTime); }
            };
            let hours = i32::from_str(&irest[0..2]).map_err(ParseError::IntConversionErr)?;
            let minutes = i32::from_str(&irest[3..5]).map_err(ParseError::IntConversionErr)?;
            rest = &irest[5..];
            minutes + hours * 60 * sign
        }
    };
    let result = FixedOffset::east_opt(utc_offset_mins * 60).and_then(|off| {
        off.ymd_opt(tm_year as i32, tm_mon, tm_mday)
            .and_hms_opt(tm_hour, tm_min, tm_sec)
            .single()
    });
    match result {
        None => Err(ParseError::InvalidDateTime),
        Some(tm) => Ok((Some(tm), rest))
    }
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
                return Err(ParseError::TooFewDigits);
            }
            let utf8_ary = str::from_utf8(&byte_ary[..idx]).map_err(ParseError::BaseUnicodeError)?;
            return Ok((Some(String::from(utf8_ary)), &m[idx..]));
        }
        if idx >= max_length {
            let utf8_ary = str::from_utf8(&byte_ary[..idx]).map_err(ParseError::BaseUnicodeError)?;
            return Ok((Some(String::from(utf8_ary)), &m[idx..]));
        }
    }
    Err(ParseError::UnexpectedEndOfInput)
}

pub fn parse_message(m: &str) -> ParseResult<Message> {
    let mut rest = m;
    take_char!(rest, '<');
    let prival = take_item!(parse_num(rest, 1, 3), rest);
    take_char!(rest, '>');
    let sev = parse_pri_val(prival)?;
    let version = take_item!(parse_num(rest, 1, 2), rest);
    if version != 1 {
        return Err(ParseError::UnsupportedVersion(version));
    }
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
        Some(s) => Some(match u32::from_str(&s) {
            Ok(n) => ProcId::Pid(n),
            Err(_) => ProcId::Name(s)
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

    Ok(Message {
        severity: sev,
        timestamp: timestamp,
        hostname: hostname,
        appname: appname,
        procid: procid,
        msgid: msgid,
        msg: msg
    })
}

#[cfg(test)]
mod tests {
    use super::parse_message;

    use message::ProcId;
    use severity::Severity;

    #[test]
    fn test_router_message() {
        let msg = parse_message(r#"<158>1 2014-08-04T18:28:43.078581+00:00 host heroku router - at=info method=GET path="/foo" host=app-name-7277.herokuapp.com request_id=e5bb3580-44b0-46d2-aad3-185263641044 fwd="50.168.96.221" dyno=web.1 connect=0ms service=2ms status=200 bytes=415"#)
            .expect("Should parse router message");
        assert_eq!(msg.severity, Severity::SEV_INFO);
        assert_eq!(msg.timestamp.map(|dt| dt.timestamp()), Some(1407176923));
        assert_eq!(msg.hostname, Some("host".to_owned()));
        assert_eq!(msg.appname, Some("heroku".to_owned()));
        assert_eq!(msg.procid, Some(ProcId::Name("router".to_owned())));
        assert_eq!(msg.msgid, None);
    }

    #[test]
    fn test_web_app_message() {
        let msg = parse_message(r#"<190>1 2014-08-04T18:28:43.015630+00:00 host app web.1 - 50.168.96.221 - - [04/Aug/2014 18:28:43] "GET /foo HTTP/1.1" 200 12 0.0019"#)
            .expect("Should parse web app message");
        assert_eq!(msg.severity, Severity::SEV_INFO);
        assert_eq!(msg.timestamp.map(|dt| dt.timestamp()), Some(1407176923));
        assert_eq!(msg.hostname, Some("host".to_owned()));
        assert_eq!(msg.appname, Some("app".to_owned()));
        assert_eq!(msg.procid, Some(ProcId::Name("web.1".to_owned())));
        assert_eq!(msg.msgid, None);
    }
}
