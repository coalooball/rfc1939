use crate::common::parse_u8_slice_to_usize_or_0;
use crate::types::command::{Dele, List, Noop, Retr, Rset, Stat, Top, Uidl};
use nom::sequence::separated_pair;
use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::digit1,
    combinator::{map, opt},
    sequence::delimited,
    sequence::{preceded, terminated, tuple},
    IResult,
};

// ################################################################################
/// STAT
/// Arguments: none
/// Examples:
///     C: STAT
// ################################################################################
pub fn stat(s: &[u8]) -> Option<Stat> {
    match stat_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn stat_parser(s: &[u8]) -> IResult<&[u8], Stat> {
    map(terminated(tag_no_case(b"STAT"), tag(b"\r\n")), |_| Stat)(s)
}

// ################################################################################
/// LIST [msg]
/// Arguments:
///     A message-number (optional)
/// Restrictions:
///     May only be given in the TRANSACTION state
/// Examples:
///     C: LIST
///     C: LIST 2
// ################################################################################
pub fn list(s: &[u8]) -> Option<List> {
    match list_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn list_parser(s: &[u8]) -> IResult<&[u8], List> {
    map(
        terminated(
            tuple((tag_no_case(b"LIST"), opt(preceded(tag(b" "), digit1)))),
            tag(b"\r\n"),
        ),
        |(_, x)| match x {
            Some(num) => List {
                msg: Some(parse_u8_slice_to_usize_or_0(num)),
            },
            None => List { msg: None },
        },
    )(s)
}

// ################################################################################
/// RETR msg
/// Arguments:
///     a message-number (required)
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Examples:
///     C: RETR 1
// ################################################################################
pub fn retr(s: &[u8]) -> Option<Retr> {
    match retr_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn retr_parser(s: &[u8]) -> IResult<&[u8], Retr> {
    map(
        delimited(
            tag_no_case(b"RETR "),
            map(digit1, parse_u8_slice_to_usize_or_0),
            tag(b"\r\n"),
        ),
        |x| Retr { msg: x },
    )(s)
}

// ################################################################################
/// DELE msg
/// Arguments:
///     a message-number (required)
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Examples:
///     C: DELE 1
// ################################################################################
pub fn dele(s: &[u8]) -> Option<Dele> {
    match dele_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn dele_parser(s: &[u8]) -> IResult<&[u8], Dele> {
    map(
        delimited(
            tag_no_case(b"DELE "),
            map(digit1, parse_u8_slice_to_usize_or_0),
            tag(b"\r\n"),
        ),
        |x| Dele { msg: x },
    )(s)
}

// ################################################################################
/// NOOP
/// Arguments: none
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Examples:
///     C: NOOP
// ################################################################################
pub fn noop(s: &[u8]) -> Option<Noop> {
    match noop_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn noop_parser(s: &[u8]) -> IResult<&[u8], Noop> {
    map(terminated(tag_no_case(b"NOOP"), tag(b"\r\n")), |_| Noop)(s)
}

// ################################################################################
/// RSET
/// Arguments: none
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Examples:
///     C: RSET
// ################################################################################
pub fn rset(s: &[u8]) -> Option<Rset> {
    match rset_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn rset_parser(s: &[u8]) -> IResult<&[u8], Rset> {
    map(terminated(tag_no_case(b"RSET"), tag(b"\r\n")), |_| Rset)(s)
}

// ################################################################################
/// TOP msg n
/// Arguments:
///     a message-number (required)
///     a non-negative number of lines (required)
/// Restrictions:
///     may only be given in the TRANSACTION state
/// Examples:
///     C: TOP 20 20
// ################################################################################
pub fn top(s: &[u8]) -> Option<Top> {
    match top_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn top_parser(s: &[u8]) -> IResult<&[u8], Top> {
    map(
        delimited(
            tag_no_case(b"TOP "),
            separated_pair(
                map(digit1, parse_u8_slice_to_usize_or_0),
                tag(b" "),
                map(digit1, parse_u8_slice_to_usize_or_0),
            ),
            tag(b"\r\n"),
        ),
        |(x, y)| Top { msg: x, n: y },
    )(s)
}

// ################################################################################
/// UIDL [msg]
/// Arguments:
///     A message-number (optional)
/// Restrictions:
///     May only be given in the TRANSACTION state
/// Examples:
///     C: UIDL
///     C: UIDL 2
// ################################################################################
pub fn uidl(s: &[u8]) -> Option<Uidl> {
    match uidl_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn uidl_parser(s: &[u8]) -> IResult<&[u8], Uidl> {
    map(
        terminated(
            tuple((tag_no_case(b"UIDL"), opt(preceded(tag(b" "), digit1)))),
            tag(b"\r\n"),
        ),
        |(_, x)| match x {
            Some(num) => Uidl {
                msg: Some(parse_u8_slice_to_usize_or_0(num)),
            },
            None => Uidl { msg: None },
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat() {
        assert_eq!(stat(b"stat\r\n"), Some(Stat));
        assert_eq!(stat(b" stat"), None);
        assert_eq!(stat(b"stat"), None);
    }

    #[test]
    fn test_list_parser() {
        assert_eq!(list_parser(b"LIST\r\n").unwrap().1, List { msg: None });
        assert_eq!(
            list_parser(b"LIST 123\r\n").unwrap().1,
            List { msg: Some(123) }
        );
    }

    #[test]
    fn test_list() {
        assert_eq!(list(b"LIST\r\n").unwrap(), List { msg: None });
        assert_eq!(list(b"LIST 2222\r\n").unwrap(), List { msg: Some(2222) });
    }

    #[test]
    fn test_retr() {
        assert_eq!(retr(b"RETR 1\r\n").unwrap(), Retr { msg: 1 });
    }

    #[test]
    fn test_dele() {
        assert_eq!(dele(b"DELE 1\r\n").unwrap(), Dele { msg: 1 });
    }

    #[test]
    fn test_noop() {
        assert_eq!(noop(b"NOOP\r\n").unwrap(), Noop);
    }

    #[test]
    fn test_rset() {
        assert_eq!(rset(b"RSET\r\n").unwrap(), Rset);
    }
    #[test]
    fn test_top() {
        assert_eq!(top(b"TOP 1 10\r\n").unwrap(), Top { msg: 1, n: 10 });
    }
    #[test]
    fn test_uidl() {
        assert_eq!(uidl(b"UIDL 1\r\n").unwrap(), Uidl { msg: Some(1) });
    }
}
