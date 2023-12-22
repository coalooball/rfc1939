use crate::common::{
    one_line_response_two_parts_parser, parse_u8_slice_to_usize_or_0, retr_message_parser,
    take_until_crlf, StatusIndicator,
};
use crate::types::response::{Dele, List, Noop, OneLineTwoParts, Retr, Rset, Stat, Top, Uidl};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::digit1,
    combinator::map,
    combinator::opt,
    multi::many1,
    sequence::tuple,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

// ################################################################################
/// STAT
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Discussion:
///     The positive response consists of "+OK" followed by a single
///     space, the number of messages in the maildrop, a single
///     space, and the size of the maildrop in octets.
// ################################################################################
pub fn stat(s: &[u8]) -> Option<Stat> {
    match stat_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn stat_parser(s: &[u8]) -> IResult<&[u8], Stat> {
    alt((
        map(
            tuple((
                map(tag_no_case(b"+OK"), |_| StatusIndicator::OK),
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
                information: &[],
            },
        ),
        map(one_line_response_two_parts_parser::<OneLineTwoParts>, |x| {
            Stat {
                status_indicator: x.left,
                number_of_messages: 0,
                size_in_octets: 0,
                information: x.right,
            }
        }),
    ))(s)
}

// ################################################################################
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
// ################################################################################
pub fn list(s: &[u8]) -> Option<List> {
    match list_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn list_parser(s: &[u8]) -> IResult<&[u8], List> {
    alt((list_multi_line_parser, list_one_line_parser))(s)
}

fn list_multi_line_parser(s: &[u8]) -> IResult<&[u8], List> {
    map(
        terminated(
            tuple((
                map(tag_no_case(b"+OK"), |_| StatusIndicator::OK),
                map(opt(preceded(tag(b" "), take_until_crlf)), |x| {
                    if let Some(msg) = x {
                        msg
                    } else {
                        &[]
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
            information: msg,
        },
    )(s)
}

fn list_one_line_parser(s: &[u8]) -> IResult<&[u8], List> {
    alt((
        map(
            terminated(
                tuple((
                    map(tag_no_case(b"+OK"), |_| StatusIndicator::OK),
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
                information: &[],
            },
        ),
        map(one_line_response_two_parts_parser::<OneLineTwoParts>, |x| {
            List {
                status_indicator: x.left,
                informations: vec![],
                information: x.right,
            }
        }),
    ))(s)
}

// ################################################################################
/// RETR msg
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Discussion:
///     If the POP3 server issues a positive response, then the
///     response given is multi-line.  After the initial +OK, the
///     POP3 server sends the message corresponding to the given
///     message-number, being careful to byte-stuff the termination
///     character (as with all multi-line responses).
/// Possible Responses:
///     +OK message follows
///     -ERR no such message
// ################################################################################
pub fn retr(s: &[u8]) -> Option<Retr> {
    match retr_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn retr_parser(s: &[u8]) -> IResult<&[u8], Retr> {
    retr_message_parser::<Retr>(s)
}

// ################################################################################
/// TOP msg n
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Discussion:
///     If the POP3 server issues a positive response, then the
///     response given is multi-line.  After the initial +OK, the
///     POP3 server sends the headers of the message, the blank
///     line separating the headers from the body, and then the
///     number of lines of the indicated message's body, being
///     careful to byte-stuff the termination character (as with
///     all multi-line responses).
///     Note that if the number of lines requested by the POP3
///     client is greater than than the number of lines in the
///     body, then the POP3 server sends the entire message.
/// Possible Responses:
///     +OK top of message follows
///     -ERR no such message
// ################################################################################
pub fn top(s: &[u8]) -> Option<Top> {
    match top_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn top_parser(s: &[u8]) -> IResult<&[u8], Top> {
    retr_message_parser::<Top>(s)
}

// ################################################################################
/// DELE msg
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Discussion:
///     The POP3 server marks the message as deleted.  Any future
///     reference to the message-number associated with the message
///     in a POP3 command generates an error.  The POP3 server does
///     not actually delete the message until the POP3 session
///     enters the UPDATE state.
/// Possible Responses:
///     +OK message deleted
///     -ERR no such message
/// Examples:
///     S: +OK message 1 deleted
///     S: -ERR message 2 already deleted
// ################################################################################
pub fn dele(s: &[u8]) -> Option<Dele> {
    match dele_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn dele_parser(s: &[u8]) -> IResult<&[u8], Dele> {
    one_line_response_two_parts_parser::<Dele>(s)
}

// ################################################################################
/// NOOP
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Discussion:
///     The POP3 server does nothing, it merely replies with a
///     positive response.
/// Possible Responses:
///     +OK
/// Examples:
///     S: +OK
// ################################################################################
pub fn noop(s: &[u8]) -> Option<Noop> {
    match noop_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn noop_parser(s: &[u8]) -> IResult<&[u8], Noop> {
    one_line_response_two_parts_parser::<Noop>(s)
}

// ################################################################################
/// RSET
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Discussion:
///     If any messages have been marked as deleted by the POP3
///     server, they are unmarked.  The POP3 server then replies
///     with a positive response.
/// Possible Responses:
///     +OK
/// Examples:
///     S: +OK maildrop has 2 messages (320 octets)
// ################################################################################
pub fn rset(s: &[u8]) -> Option<Rset> {
    match rset_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn rset_parser(s: &[u8]) -> IResult<&[u8], Rset> {
    one_line_response_two_parts_parser::<Rset>(s)
}

// ################################################################################
/// UIDL [msg]
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Discussion:
///     If an argument was given and the POP3 server issues a positive
///     response with a line containing information for that message.
///     This line is called a "unique-id listing" for that message.

///     If no argument was given and the POP3 server issues a positive
///     response, then the response given is multi-line.  After the
///     initial +OK, for each message in the maildrop, the POP3 server
///     responds with a line containing information for that message.
///     This line is called a "unique-id listing" for that message.

///     In order to simplify parsing, all POP3 servers are required to
///     use a certain format for unique-id listings.  A unique-id
///     listing consists of the message-number of the message,
///     followed by a single space and the unique-id of the message.
///     No information follows the unique-id in the unique-id listing.

///     The unique-id of a message is an arbitrary server-determined
///     string, consisting of one to 70 characters in the range 0x21
///     to 0x7E, which uniquely identifies a message within a
///     maildrop and which persists across sessions.  This
///     persistence is required even if a session ends without
///     entering the UPDATE state.  The server should never reuse an
///     unique-id in a given maildrop, for as long as the entity
///     using the unique-id exists.

///     Note that messages marked as deleted are not listed.

///     While it is generally preferable for server implementations
///     to store arbitrarily assigned unique-ids in the maildrop,
///     this specification is intended to permit unique-ids to be
///     calculated as a hash of the message.  Clients should be able
///     to handle a situation where two identical copies of a
///     message in a maildrop have the same unique-id.
// ################################################################################
pub fn uidl(s: &[u8]) -> Option<Uidl> {
    match uidl_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn uidl_parser(s: &[u8]) -> IResult<&[u8], Uidl> {
    alt((uidl_multi_line_parser, uidl_one_line_parser))(s)
}

fn uidl_multi_line_parser(s: &[u8]) -> IResult<&[u8], Uidl> {
    map(
        terminated(
            tuple((
                map(tag_no_case(b"+OK"), |_| StatusIndicator::OK),
                map(opt(preceded(tag(b" "), take_until_crlf)), |x| {
                    if let Some(msg) = x {
                        msg
                    } else {
                        &[]
                    }
                }),
                many1(preceded(
                    tag(b"\r\n"),
                    separated_pair(
                        map(digit1, parse_u8_slice_to_usize_or_0),
                        tag(b" "),
                        take_until_crlf,
                    ),
                )),
            )),
            tag(b"\r\n.\r\n"),
        ),
        |(si, information, infos)| Uidl {
            status_indicator: si,
            informations: infos,
            information: information,
        },
    )(s)
}

fn uidl_one_line_parser(s: &[u8]) -> IResult<&[u8], Uidl> {
    alt((
        map(
            terminated(
                tuple((
                    map(tag_no_case(b"+OK"), |_| StatusIndicator::OK),
                    tag(b" "),
                    map(digit1, parse_u8_slice_to_usize_or_0),
                    tag(b" "),
                    take_until_crlf,
                )),
                tag(b"\r\n"),
            ),
            |(si, _, num, _, size)| Uidl {
                status_indicator: si,
                informations: vec![(num, size)],
                information: &[],
            },
        ),
        map(one_line_response_two_parts_parser::<OneLineTwoParts>, |x| {
            Uidl {
                status_indicator: x.left,
                informations: vec![],
                information: x.right,
            }
        }),
    ))(s)
}

#[test]
fn test_stat_parser() {
    assert_eq!(
        stat_parser(b"+OK 2 320\r\n").unwrap().1,
        Stat {
            status_indicator: StatusIndicator::OK,
            number_of_messages: 2,
            size_in_octets: 320,
            information: &[]
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stat() {
        assert_eq!(
            stat(b"+OK 2 320\r\n").unwrap(),
            Stat {
                status_indicator: StatusIndicator::OK,
                number_of_messages: 2,
                size_in_octets: 320,
                information: &[]
            }
        );
        assert_eq!(
            stat(b"-ERR failed\r\n").unwrap(),
            Stat {
                status_indicator: StatusIndicator::ERR,
                number_of_messages: 0,
                size_in_octets: 0,
                information: b"failed"
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
                information: b"2 messages (320 octets)"
            }
        );
        assert_eq!(
            list_multi_line_parser(b"+OK\r\n1 120\r\n2 200\r\n.\r\n")
                .unwrap()
                .1,
            List {
                status_indicator: StatusIndicator::OK,
                informations: vec![(1, 120), (2, 200)],
                information: b""
            }
        );
        assert_eq!(
            list_one_line_parser(b"+OK 1 60178\r\n").unwrap().1,
            List {
                status_indicator: StatusIndicator::OK,
                informations: vec![(1, 60178)],
                information: &[]
            }
        );
        assert_eq!(
            list_one_line_parser(b"-ERR Syntax error\r\n").unwrap().1,
            List {
                status_indicator: StatusIndicator::ERR,
                informations: vec![],
                information: b"Syntax error"
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
                information: b"2 messages (320 octets)"
            }
        );
        assert_eq!(
            list(b"-ERR Syntax error\r\n").unwrap(),
            List {
                status_indicator: StatusIndicator::ERR,
                informations: vec![],
                information: b"Syntax error"
            }
        );
    }

    #[test]
    fn test_retr() {
        assert_eq!(
            retr(b"+OK 120 octets\r\n<the POP3 server sends the entire message here>\r\n.\r\n")
                .unwrap(),
            Retr {
                status_indicator: StatusIndicator::OK,
                message: Some(b"<the POP3 server sends the entire message here>"),
                information: b"120 octets"
            }
        );
    }

    #[test]
    fn test_dele() {
        assert_eq!(
            dele(b"+OK message 1 deleted\r\n").unwrap(),
            Dele {
                status_indicator: StatusIndicator::OK,
                information: b"message 1 deleted"
            }
        );
    }

    #[test]
    fn test_noop() {
        assert_eq!(
            noop(b"+OK\r\n").unwrap(),
            Noop {
                status_indicator: StatusIndicator::OK,
                information: b""
            }
        );
    }

    #[test]
    fn test_rset() {
        assert_eq!(
            rset(b"+OK core mail\r\n").unwrap(),
            Rset {
                status_indicator: StatusIndicator::OK,
                information: b"core mail"
            }
        );
    }
    #[test]
    fn test_top() {
        assert_eq!(
            top(b"+OK 1364\r\n58.87.109.78\r\n.\r\n").unwrap(),
            Top {
                status_indicator: StatusIndicator::OK,
                message: Some(b"58.87.109.78"),
                information: b"1364"
            }
        );
    }

    #[test]
    fn test_uidl() {
        assert_eq!(
            uidl(b"+OK\r\n1 whqtswO00WBw418f9t5JxYwZ\r\n2 QhdPYR:00WBw1Ph7x7\r\n.\r\n").unwrap(),
            Uidl {
                status_indicator: StatusIndicator::OK,
                informations: vec![(1, b"whqtswO00WBw418f9t5JxYwZ"), (2, b"QhdPYR:00WBw1Ph7x7")],
                information: &vec![]
            }
        );
        assert_eq!(
            uidl(b"+OK 1 whqtswO00WBw418f9t5JxYwZ%\r\n").unwrap(),
            Uidl {
                status_indicator: StatusIndicator::OK,
                informations: vec![(1, b"whqtswO00WBw418f9t5JxYwZ%")],
                information: &vec![]
            }
        );
        assert_eq!(
            uidl(b"-ERR Syntax error\r\n").unwrap(),
            Uidl {
                status_indicator: StatusIndicator::ERR,
                informations: vec![],
                information: b"Syntax error"
            }
        );
    }
}
