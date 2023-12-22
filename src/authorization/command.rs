use crate::common::take_until_crlf_consume_crlf;
use crate::types::command::{Pass, User};
use nom::{bytes::complete::tag_no_case, combinator::map, sequence::preceded, IResult};

// ################################################################################
/// USER name
/// Arguments:
///     a string identifying a mailbox (required)
/// Restrictions:
///     a string identifying a mailbox (required)
/// Examples:
///     C: USER frated
// ################################################################################
pub fn user(s: &[u8]) -> Option<User> {
    match user_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn user_parser(s: &[u8]) -> IResult<&[u8], User> {
    map(
        preceded(tag_no_case(b"USER "), take_until_crlf_consume_crlf),
        |name| User { name: name },
    )(s)
}

// ################################################################################
/// PASS string
/// Arguments:
///     a server/mailbox-specific password (required)
/// Restrictions:
///     a string identifying a mailbox (required)
/// Examples:
///     C: USER frated
// ################################################################################
pub fn pass(s: &[u8]) -> Option<Pass> {
    match pass_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn pass_parser(s: &[u8]) -> IResult<&[u8], Pass> {
    map(
        preceded(tag_no_case(b"PASS "), take_until_crlf_consume_crlf),
        |string| Pass { string: string },
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        assert_eq!(user(b"USER name\r\n").unwrap(), User { name: b"name" })
    }

    #[test]
    fn test_pass() {
        assert_eq!(pass(b"PASS pwd\r\n").unwrap(), Pass { string: b"pwd" })
    }
}
