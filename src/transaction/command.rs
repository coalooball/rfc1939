use crate::types::command::Stat;
use nom::{
    bytes::complete::{tag, tag_no_case},
    combinator::map,
    sequence::terminated,
    IResult,
};

pub(crate) fn stat_parser(s: &[u8]) -> IResult<&[u8], Stat> {
    map(terminated(tag_no_case(b"STAT"), tag(b"\r\n")), |_| Stat)(s)
}

/// STAT
/// Arguments: none
/// Examples:
///     C: STAT
pub fn stat(s: &[u8]) -> Option<Stat> {
    match stat_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

#[test]
fn test_stat() {
    assert_eq!(stat(b"stat\r\n"), Some(Stat));
    assert_eq!(stat(b" stat"), None);
    assert_eq!(stat(b"stat"), None);
}
