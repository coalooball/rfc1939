use crate::common::take_until_crlf_consume_crlf;
use crate::types::command::User;
use nom::{bytes::complete::tag_no_case, combinator::map, sequence::preceded, IResult};

// ################################################################################
/// USER name
/// Arguments: none
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        assert_eq!(user(b"USER name\r\n").unwrap(), User { name: b"name" })
    }
}
