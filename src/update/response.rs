use crate::common::one_line_response_two_parts_parser;
use crate::types::response::Quit;
use nom::IResult;

// ################################################################################
/// QUIT
/// Restrictions: none
/// Discussion:
///     The POP3 server removes all messages marked as deleted
///     from the maildrop and replies as to the status of this
///     operation.  If there is an error, such as a resource
///     shortage, encountered while removing messages, the
///     maildrop may result in having some or none of the messages
///     marked as deleted be removed.  In no case may the server
///     remove any messages not marked as deleted.

///     Whether the removal was successful or not, the server
///     then releases any exclusive-access lock on the maildrop
///     and closes the TCP connection.
/// Possible Responses:
///     +OK
///     -ERR some deleted messages not removed
/// Examples:
///     S: +OK dewey POP3 server signing off (maildrop empty)
// ################################################################################
pub fn quit(s: &[u8]) -> Option<Quit> {
    match quit_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

pub(crate) fn quit_parser(s: &[u8]) -> IResult<&[u8], Quit> {
    one_line_response_two_parts_parser::<Quit>(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::StatusIndicator;

    #[test]
    fn test_quit() {
        assert_eq!(
            quit(b"-ERR some deleted messages not removed\r\n").unwrap(),
            Quit {
                status_indicator: StatusIndicator::ERR,
                information: b"some deleted messages not removed"
            }
        );
    }
}
