//! This module implements a parser for Heroku (Logplex) syslog messages, which is useful when implementing a
//! [Logplex HTTP Drain](https://github.com/heroku/logplex/blob/master/doc/README.http_drains.md).
//!
//! These syslog messages are similar to [RFC5424](https://tools.ietf.org/html/rfc5424) messages,
//! with the notable exception that they leave out `STRUCTURED-DATA` but do not replace it with a `NILVALUE`.
//!
//! Usually, you'll just use the `FromStr` trait on the `Message` struct to parse a message.
//!
//! # Example
//!
//! ```
//! use syslog_heroku::Message;
//!
//! fn main() {
//!     let msg = "<45>1 2018-02-28T09:30:53.345547+00:00 host heroku web.1 - Process exited with status 143"
//!         .parse::<Message>().unwrap();
//!     println!("{:?} {:?} {:?}", msg.severity, msg.hostname, msg.msg);
//! }
//! ```
#[macro_use] extern crate failure;
extern crate chrono;

mod message;
mod severity;
mod parser;

pub use severity::Severity;
pub use message::Message;
pub use parser::ParseError;
