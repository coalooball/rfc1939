use crate::common::{take_until_crlf, take_until_crlf_consume_crlf, take_until_sp};
use crate::types::command::{Apop, Pass, User};
use nom::{
    bytes::complete::tag, bytes::complete::tag_no_case, combinator::map, sequence::delimited,
    sequence::preceded, sequence::separated_pair, IResult,
};

// ################################################################################
/// USER *name*
/// 
/// **Arguments**
/// 
/// A string identifying a mailbox (required)
/// 
/// **Restrictions**
/// 
/// may only be given in the AUTHORIZATION state after the POP3
/// greeting or after an unsuccessful USER or PASS command
/// 
/// **Examples**
/// 
/// ```rust
/// // C: USER frated
/// use rfc1939::authorization::command::user;
/// use rfc1939::types::command::User;
/// assert_eq!(user(b"USER name\r\n").unwrap(), User { name: b"name" })
/// ```
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
/// PASS *string*
/// 
/// **Arguments**
/// 
/// a server/mailbox-specific password (required)
/// 
/// **Restrictions**
/// 
/// may only be given in the AUTHORIZATION state immediately
/// after a successful USER command
/// 
/// **Examples**
/// 
/// ```rust
/// // C: PASS secret
/// use rfc1939::authorization::command::pass;
/// use rfc1939::types::command::Pass;
/// assert_eq!(pass(b"PASS secret\r\n").unwrap(), Pass { string: b"secret" })
/// ```
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

// ################################################################################
/// APOP *name* *digest*
/// 
/// **Arguments**
/// 
/// a string identifying a mailbox and a MD5 digest string
/// (both required)
/// 
/// **Restrictions**
/// 
/// may only be given in the AUTHORIZATION state after the POP3
/// greeting or after an unsuccessful USER or PASS command
/// 
/// **Examples**
/// 
/// ```rust
/// use rfc1939::authorization::command::apop;
/// use rfc1939::types::command::Apop;
/// // C: APOP mrose c4c9334bac560ecc979e58001b3e22fb
/// assert_eq!(
///     apop(b"APOP mrose c4c9334bac560ecc979e58001b3e22fb\r\n").unwrap(),
///     Apop {
///         name: b"mrose",
///         digest: b"c4c9334bac560ecc979e58001b3e22fb"
///     }
/// )
/// ```
// ################################################################################
pub fn apop(s: &[u8]) -> Option<Apop> {
    match apop_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn apop_parser(s: &[u8]) -> IResult<&[u8], Apop> {
    map(
        delimited(
            tag_no_case(b"APOP "),
            separated_pair(take_until_sp, tag(b" "), take_until_crlf),
            tag(b"\r\n"),
        ),
        |(x, y)| Apop { name: x, digest: y },
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

    #[test]
    fn test_apop() {
        assert_eq!(
            apop(b"APOP mrose c4c9334bac560ecc979e58001b3e22fb\r\n").unwrap(),
            Apop {
                name: b"mrose",
                digest: b"c4c9334bac560ecc979e58001b3e22fb"
            }
        )
    }
}
