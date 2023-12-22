//! ## A library for parsing byte data streams implemented according to RFC 1939.
//! ###### POP3 Command Summary
//！    Minimal POP3 Commands:
//！
//！        USER name               valid in the AUTHORIZATION state
//！        PASS string
//！        QUIT
//！
//！        STAT                    valid in the TRANSACTION state
//！        LIST [msg]
//！        RETR msg
//！        DELE msg
//！        NOOP
//！        RSET
//！        QUIT
//！
//！    Optional POP3 Commands:
//！
//！        APOP name digest        valid in the AUTHORIZATION state
//！
//！        TOP msg n               valid in the TRANSACTION state
//！        UIDL [msg]
//! ##### Usage
//! All commands and reponses are categorized into three sections:
//! AUTHORIZATION, TRANSACTION and UPDATE.
//!
//! ###### Command Examples
//! ```rust
//! use rfc1939::authorization::command::*;
//! use rfc1939::transaction::command::*;
//! use rfc1939::types::command::*;
//!
//! # fn main() {
//! assert_eq!(user(b"USER name\r\n").unwrap(), User { name: b"name" });
//! assert_eq!(pass(b"PASS pwd\r\n").unwrap(), Pass { string: b"pwd" });
//! assert_eq!(
//!     apop(b"APOP mrose c4c9334bac560ecc979e58001b3e22fb\r\n").unwrap(),
//!     Apop {
//!         name: b"mrose",
//!         digest: b"c4c9334bac560ecc979e58001b3e22fb"
//!     }
//! );
//! assert_eq!(stat(b"stat\r\n").unwrap(), Stat);
//! assert_eq!(list(b"LIST 2222\r\n").unwrap(), List { msg: Some(2222) });
//! assert_eq!(retr(b"RETR 1\r\n").unwrap(), Retr { msg: 1 });
//! assert_eq!(dele(b"DELE 1\r\n").unwrap(), Dele { msg: 1 });
//! assert_eq!(noop(b"NOOP\r\n").unwrap(), Noop);
//! assert_eq!(rset(b"RSET\r\n").unwrap(), Rset);
//! assert_eq!(top(b"TOP 1 10\r\n").unwrap(), Top { msg: 1, n: 10 });
//! assert_eq!(uidl(b"UIDL 1\r\n").unwrap(), Uidl { msg: Some(1) });
//! # }
//! ```
//! ###### Response Example
//! ```rust
//! use rfc1939::transaction::response::retr;
//! use rfc1939::types::response::Retr;
//! use rfc1939::common::StatusIndicator;
//!
//! # fn main() {
//! assert_eq!(
//!     retr(b"+OK 120 octets\r\n<the POP3 server sends the entire message here>\r\n.\r\n")
//!         .unwrap(),
//!     Retr {
//!         status_indicator: StatusIndicator::OK,
//!         message: Some(b"<the POP3 server sends the entire message here>"),
//!         information: b"120 octets"
//!     }
//! );
//! # }
//! ```
// State
pub mod authorization;
pub mod transaction;
pub mod update;

pub mod common;
pub mod types;
