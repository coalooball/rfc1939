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
/// Possible Responses:
///     +OK name is a valid mailbox
///     -ERR never heard of mailbox name
/// Examples:
///     S: +OK mrose is a real hoopy frood
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

// ################################################################################
/// PASS string
/// Restrictions:
///     may only be given in the AUTHORIZATION state immediately
///     after a successful USER command
/// Discussion:
///     When the client issues the PASS command, the POP3 server
///     uses the argument pair from the USER and PASS commands to
///     determine if the client should be given access to the
///     appropriate maildrop.

///     Since the PASS command has exactly one argument, a POP3
///     server may treat spaces in the argument as part of the
///     password, instead of as argument separators.
/// Possible Responses:
///     +OK maildrop locked and ready
///     -ERR invalid password
///     -ERR unable to lock maildrop
/// Examples:
///     S: -ERR maildrop already locked
///     S: +OK mrose's maildrop has 2 messages (320 octets)
// ################################################################################
pub fn pass(s: &[u8]) -> Option<Pass> {
    match pass_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn pass_parser(s: &[u8]) -> IResult<&[u8], Pass> {
    one_line_response_two_parts_parser::<Pass>(s)
}

// ################################################################################
/// APOP name digest
/// Restrictions:
///     may only be given in the AUTHORIZATION state after the POP3
///     greeting or after an unsuccessful USER or PASS command
/// Possible Responses:
///     +OK maildrop locked and ready
///     -ERR permission denied
/// Examples:
///     S: +OK POP3 server ready <1896.697170952@dbc.mtview.ca.us>
///     C: APOP mrose c4c9334bac560ecc979e58001b3e22fb
///     S: +OK maildrop has 1 message (369 octets)
// ################################################################################
pub fn apop(s: &[u8]) -> Option<Apop> {
    match apop_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn apop_parser(s: &[u8]) -> IResult<&[u8], Apop> {
    one_line_response_two_parts_parser::<Apop>(s)
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
                information: b"POP3 server ready"
            }
        )
    }

    #[test]
    fn test_greeting() {
        assert_eq!(
            greeting(b"+OK POP3 server ready\r\n").unwrap(),
            Greeting {
                status_indicator: StatusIndicator::OK,
                information: b"POP3 server ready"
            }
        )
    }

    #[test]
    fn test_quit() {
        assert_eq!(
            quit(b"+OK dewey POP3 server signing off\r\n").unwrap(),
            Quit {
                status_indicator: StatusIndicator::OK,
                information: b"dewey POP3 server signing off"
            }
        )
    }

    #[test]
    fn test_user() {
        assert_eq!(
            user(b"+OK successfully\r\n").unwrap(),
            User {
                status_indicator: StatusIndicator::OK,
                information: b"successfully"
            }
        )
    }

    #[test]
    fn test_pass() {
        assert_eq!(
            pass(b"+OK successfully\r\n").unwrap(),
            Pass {
                status_indicator: StatusIndicator::OK,
                information: b"successfully"
            }
        )
    }

    #[test]
    fn test_apop() {
        assert_eq!(
            apop(b"+OK maildrop has 1 message (369 octets)\r\n").unwrap(),
            Apop {
                status_indicator: StatusIndicator::OK,
                information: b"maildrop has 1 message (369 octets)"
            }
        )
    }
}
