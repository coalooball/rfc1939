#[derive(Debug, PartialEq)]
pub struct Stat;

/// A message-number (optional)
#[derive(Debug, PartialEq)]
pub struct List {
    pub msg: Option<usize>,
}

/// A message-number (required)
#[derive(Debug, PartialEq)]
pub struct Retr {
    pub msg: usize,
}

/// A message-number (required)
#[derive(Debug, PartialEq)]
pub struct Dele {
    pub msg: usize,
}

#[derive(Debug, PartialEq)]
pub struct Noop;

#[derive(Debug, PartialEq)]
pub struct Rset;

#[derive(Debug, PartialEq)]
pub struct Quit;

/// A message-number (required)
/// 
/// A non-negative number of lines (required)
#[derive(Debug, PartialEq)]
pub struct Top {
    pub msg: usize,
    pub n: usize,
}

/// A message-number (optional)
#[derive(Debug, PartialEq)]
pub struct Uidl {
    pub msg: Option<usize>,
}

/// A string identifying a mailbox (required)
#[derive(Debug, PartialEq)]
pub struct User<'a> {
    pub name: &'a [u8],
}

/// A server/mailbox-specific password (required)
#[derive(Debug, PartialEq)]
pub struct Pass<'a> {
    pub string: &'a [u8],
}

/// A string identifying a mailbox (required)
/// 
/// A MD5 digest string (required)
#[derive(Debug, PartialEq)]
pub struct Apop<'a> {
    pub name: &'a [u8],
    pub digest: &'a [u8],
}
