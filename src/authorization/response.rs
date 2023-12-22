use crate::common::*;
use crate::types::response::*;
use nom::IResult;

// ################################################################################
/// Greeting
/// Once the TCP connection has been opened by a POP3 client, the POP3
/// server issues a one line greeting.
// ################################################################################
pub fn greeting(s: &[u8]) -> Option<Greeting> {
    match greeting_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn greeting_parser(s: &[u8]) -> IResult<&[u8], Greeting> {
    one_line_response_two_parts_parser::<Greeting>(s)
}

// ################################################################################
/// QUIT
/// The QUIT command when used in the AUTHORIZATION state
// ################################################################################
pub fn quit(s: &[u8]) -> Option<Quit> {
    match quit_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn quit_parser(s: &[u8]) -> IResult<&[u8], Quit> {
    one_line_response_two_parts_parser::<Quit>(s)
}

// ################################################################################
/// USER name
/// Restrictions:
///     may only be given in the AUTHORIZATION state after the POP3
///     greeting or after an unsuccessful USER or PASS command
/// Discussion:
///     To authenticate using the USER and PASS command
///     combination, the client must first issue the USER
///     command.  If the POP3 server responds with a positive
///     status indicator ("+OK"), then the client may issue
///     either the PASS command to complete the authentication,
///     or the QUIT command to terminate the POP3 session.  If
///     the POP3 server responds with a negative status indicator
///     ("-ERR") to the USER command, then the client may either
///     issue a new authentication command or may issue the QUIT
///     command.

///     The server may return a positive response even though no
///     such mailbox exists.  The server may return a negative
///     response if mailbox exists, but does not permit plaintext
///     password authentication.
///     Possible Responses:
///         +OK name is a valid mailbox
///         -ERR never heard of mailbox name
///     Examples:
///         S: +OK mrose is a real hoopy frood
// ################################################################################
pub fn user(s: &[u8]) -> Option<User> {
    match user_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn user_parser(s: &[u8]) -> IResult<&[u8], User> {
    one_line_response_two_parts_parser::<User>(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_greeting_parser() {
        assert_eq!(
            greeting_parser(b"+OK POP3 server ready\r\n").unwrap().1,
            Greeting {
                status_indicator: StatusIndicator::OK,
                message: b"POP3 server ready"
            }
        )
    }

    #[test]
    fn test_greeting() {
        assert_eq!(
            greeting(b"+OK POP3 server ready\r\n").unwrap(),
            Greeting {
                status_indicator: StatusIndicator::OK,
                message: b"POP3 server ready"
            }
        )
    }

    #[test]
    fn test_quit() {
        assert_eq!(
            quit(b"+OK dewey POP3 server signing off\r\n").unwrap(),
            Quit {
                status_indicator: StatusIndicator::OK,
                message: b"dewey POP3 server signing off"
            }
        )
    }

    #[test]
    fn test_user() {
        assert_eq!(
            user(b"+OK successfully\r\n").unwrap(),
            User {
                status_indicator: StatusIndicator::OK,
                message: b"successfully"
            }
        )
    }
}
