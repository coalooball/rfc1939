//! Once the client has successfully identified itself to the POP3 server
//! and the POP3 server has locked and opened the appropriate maildrop,
//! the POP3 session is now in the TRANSACTION state.

pub mod command;
pub mod response;
