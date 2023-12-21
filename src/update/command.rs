use crate::types::command::Quit;
use nom::{
    bytes::complete::{tag, tag_no_case},
    combinator::map,
    sequence::terminated,
    IResult,
};

// ################################################################################
/// QUIT
/// Arguments: none
/// Restrictions: none
/// Examples:
///     C: QUIT
// ################################################################################
pub fn quit(s: &[u8]) -> Option<Quit> {
    match quit_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn quit_parser(s: &[u8]) -> IResult<&[u8], Quit> {
    map(terminated(tag_no_case(b"QUIT"), tag(b"\r\n")), |_| Quit)(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quit() {
        assert_eq!(quit(b"QUIT\r\n").unwrap(), Quit);
    }
}
