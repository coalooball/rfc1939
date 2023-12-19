use nom::{
    bytes::complete::{tag, take_until1},
    sequence::terminated,
    IResult,
};

pub fn take_untill_crlf(s: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(take_until1("\r\n"), tag(b"\r\n"))(s)
}

#[test]
fn test_take_untill_crlf() {
    assert_eq!(take_untill_crlf(b"1234567\r\n").unwrap().1, b"1234567");
}
