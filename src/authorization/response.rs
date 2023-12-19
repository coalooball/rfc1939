use crate::common::take_untill_crlf;
use crate::types::{Greeting, StatusIndicator};
use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::tuple, IResult};

fn greeting_parser(s: &[u8]) -> IResult<&[u8], Greeting> {
    map(
        tuple((
            alt((
                map(tag(b"+OK"), |_| StatusIndicator::OK),
                map(tag(b"-ERR"), |_| StatusIndicator::ERR),
            )),
            tag(b" "),
            take_untill_crlf,
        )),
        |(si, _, message)| Greeting {
            status_indicator: si,
            message: message.to_vec(),
        },
    )(s)
}

pub fn greeting(s: &[u8]) -> Option<Greeting> {
    match greeting_parser(s) {
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
