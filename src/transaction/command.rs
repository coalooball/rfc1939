use crate::common::parse_u8_slice_to_usize_or_0;
use crate::types::command::{List, Stat};
use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::digit1,
    combinator::{map, opt},
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
                message_number: Some(parse_u8_slice_to_usize_or_0(num)),
            },
            None => List {
                message_number: None,
            },
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
        assert_eq!(
            list_parser(b"LIST\r\n").unwrap().1,
            List {
                message_number: None
            }
        );
        assert_eq!(
            list_parser(b"LIST 123\r\n").unwrap().1,
            List {
                message_number: Some(123)
            }
        );
    }

    #[test]
    fn test_list() {
        assert_eq!(
            list(b"LIST\r\n").unwrap(),
            List {
                message_number: None
            }
        );
        assert_eq!(
            list(b"LIST 2222\r\n").unwrap(),
            List {
                message_number: Some(2222)
            }
        );
    }
}
