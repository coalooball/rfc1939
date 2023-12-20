use crate::common::*;
use crate::types::*;
use nom::IResult;

fn greeting_parser(s: &[u8]) -> IResult<&[u8], Greeting> {
    one_line_response_two_parts_parser::<Greeting>(s)
}

/// Once the TCP connection has been opened by a POP3 client, the POP3
/// server issues a one line greeting.
pub fn greeting(s: &[u8]) -> Option<Greeting> {
    match greeting_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

fn quit_parser(s: &[u8]) -> IResult<&[u8], Quit> {
    one_line_response_two_parts_parser::<Quit>(s)
}

/// The QUIT command when used in the AUTHORIZATION state:
pub fn quit(s: &[u8]) -> Option<Quit> {
    match quit_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

#[test]
fn test_greeting_parser() {
    assert_eq!(
        greeting_parser(b"+OK POP3 server ready\r\n").unwrap().1,
        Greeting {
            status_indicator: StatusIndicator::OK,
            message: b"POP3 server ready".to_vec()
        }
    )
}

#[test]
fn test_greeting() {
    assert_eq!(
        greeting(b"+OK POP3 server ready\r\n").unwrap(),
        Greeting {
            status_indicator: StatusIndicator::OK,
            message: b"POP3 server ready".to_vec()
        }
    )
}

#[test]
fn test_quit() {
    assert_eq!(
        quit(b"+OK dewey POP3 server signing off\r\n").unwrap(),
        Quit {
            status_indicator: StatusIndicator::OK,
            message: b"dewey POP3 server signing off".to_vec()
        }
    )
}
