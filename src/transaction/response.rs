use crate::common::{
    one_line_response_two_parts_parser, parse_u8_slice_to_usize_or_0, take_until_crlf,
    StatusIndicator,
};
use crate::types::response::{List, OneLineTwoParts, Stat};
use nom::multi::many1;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    combinator::opt, sequence::tuple, IResult,
};

pub(crate) fn stat_parser(s: &[u8]) -> IResult<&[u8], Stat> {
    alt((
        map(
            tuple((
                map(tag(b"+OK"), |_| StatusIndicator::OK),
                tag(b" "),
                digit1,
                tag(b" "),
                digit1,
                tag(b"\r\n"),
            )),
            |(si, _, num, _, size, _): (StatusIndicator, _, &[u8], _, &[u8], _)| Stat {
                status_indicator: si,
                number_of_messages: parse_u8_slice_to_usize_or_0(num),
                size_in_octets: parse_u8_slice_to_usize_or_0(size),
                message: vec![],
            },
        ),
        map(one_line_response_two_parts_parser::<OneLineTwoParts>, |x| {
            Stat {
                status_indicator: x.left,
                number_of_messages: 0,
                size_in_octets: 0,
                message: x.right,
            }
        }),
    ))(s)
}

/// STAT
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Discussion:
///     The positive response consists of "+OK" followed by a single
///     space, the number of messages in the maildrop, a single
///     space, and the size of the maildrop in octets.
pub fn stat(s: &[u8]) -> Option<Stat> {
    match stat_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

fn list_multi_line_parser(s: &[u8]) -> IResult<&[u8], List> {
    map(
        terminated(
            tuple((
                map(tag(b"+OK"), |_| StatusIndicator::OK),
                map(opt(preceded(tag(b" "), take_until_crlf)), |x| {
                    if let Some(msg) = x {
                        msg.to_vec()
                    } else {
                        vec![]
                    }
                }),
                many1(preceded(
                    tag(b"\r\n"),
                    separated_pair(
                        map(digit1, parse_u8_slice_to_usize_or_0),
                        tag(b" "),
                        map(digit1, parse_u8_slice_to_usize_or_0),
                    ),
                )),
            )),
            tag(b"\r\n.\r\n"),
        ),
        |(si, msg, infos)| List {
            status_indicator: si,
            informations: infos,
            message: msg,
        },
    )(s)
}

fn list_one_line_parser(s: &[u8]) -> IResult<&[u8], List> {
    alt((
        map(
            terminated(
                tuple((
                    map(tag(b"+OK"), |_| StatusIndicator::OK),
                    tag(b" "),
                    map(digit1, parse_u8_slice_to_usize_or_0),
                    tag(b" "),
                    map(digit1, parse_u8_slice_to_usize_or_0),
                )),
                tag(b"\r\n"),
            ),
            |(si, _, num, _, size)| List {
                status_indicator: si,
                informations: vec![(num, size)],
                message: vec![],
            },
        ),
        map(one_line_response_two_parts_parser::<OneLineTwoParts>, |x| {
            List {
                status_indicator: x.left,
                informations: vec![],
                message: x.right,
            }
        }),
    ))(s)
}

pub(crate) fn list_parser(s: &[u8]) -> IResult<&[u8], List> {
    alt((list_multi_line_parser, list_one_line_parser))(s)
}

/// LIST [msg]
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Discussion:
///     If an argument was given and the POP3 server issues a
///     positive response with a line containing information for
///     that message.  This line is called a "scan listing" for
///     that message.

///     If no argument was given and the POP3 server issues a
///     positive response, then the response given is multi-line.
///     After the initial +OK, for each message in the maildrop,
///     the POP3 server responds with a line containing
///     information for that message.  This line is also called a
///     "scan listing" for that message.  If there are no
///     messages in the maildrop, then the POP3 server responds
///     with no scan listings--it issues a positive response
///     followed by a line containing a termination octet and a
///     CRLF pair.

///     In order to simplify parsing, all POP3 servers are
///     required to use a certain format for scan listings.  A
///     scan listing consists of the message-number of the
///     message, followed by a single space and the exact size of
///     the message in octets.  Methods for calculating the exact
///     size of the message are described in the "Message Format"
///     section below.  This memo makes no requirement on what
///     follows the message size in the scan listing.  Minimal
///     implementations should just end that line of the response
///     with a CRLF pair.  More advanced implementations may
///     include other information, as parsed from the message.
pub fn list(s: &[u8]) -> Option<List> {
    match list_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

#[test]
fn test_stat_parser() {
    assert_eq!(
        stat_parser(b"+OK 2 320\r\n").unwrap().1,
        Stat {
            status_indicator: StatusIndicator::OK,
            number_of_messages: 2,
            size_in_octets: 320,
            message: vec![]
        }
    )
}

#[test]
fn test_stat() {
    assert_eq!(
        stat(b"+OK 2 320\r\n").unwrap(),
        Stat {
            status_indicator: StatusIndicator::OK,
            number_of_messages: 2,
            size_in_octets: 320,
            message: vec![]
        }
    );
    assert_eq!(
        stat(b"-ERR failed\r\n").unwrap(),
        Stat {
            status_indicator: StatusIndicator::ERR,
            number_of_messages: 0,
            size_in_octets: 0,
            message: b"failed".to_vec()
        }
    )
}

#[test]
fn test_list_parser() {
    assert_eq!(
        list_multi_line_parser(b"+OK 2 messages (320 octets)\r\n1 120\r\n2 200\r\n.\r\n")
            .unwrap()
            .1,
        List {
            status_indicator: StatusIndicator::OK,
            informations: vec![(1, 120), (2, 200)],
            message: b"2 messages (320 octets)".to_vec()
        }
    );
    assert_eq!(
        list_multi_line_parser(b"+OK\r\n1 120\r\n2 200\r\n.\r\n")
            .unwrap()
            .1,
        List {
            status_indicator: StatusIndicator::OK,
            informations: vec![(1, 120), (2, 200)],
            message: b"".to_vec()
        }
    );
    assert_eq!(
        list_one_line_parser(b"+OK 1 60178\r\n").unwrap().1,
        List {
            status_indicator: StatusIndicator::OK,
            informations: vec![(1, 60178)],
            message: vec![]
        }
    );
    assert_eq!(
        list_one_line_parser(b"-ERR Syntax error\r\n").unwrap().1,
        List {
            status_indicator: StatusIndicator::ERR,
            informations: vec![],
            message: b"Syntax error".to_vec()
        }
    );
}

#[test]
fn test_list() {
    assert_eq!(
        list(b"+OK 2 messages (320 octets)\r\n1 120\r\n2 200\r\n.\r\n").unwrap(),
        List {
            status_indicator: StatusIndicator::OK,
            informations: vec![(1, 120), (2, 200)],
            message: b"2 messages (320 octets)".to_vec()
        }
    );
    assert_eq!(
        list(b"-ERR Syntax error\r\n").unwrap(),
        List {
            status_indicator: StatusIndicator::ERR,
            informations: vec![],
            message: b"Syntax error".to_vec()
        }
    );
}
