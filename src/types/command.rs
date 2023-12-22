#[derive(Debug, PartialEq)]
pub struct Stat;

/// A message-number (optional)
#[derive(Debug, PartialEq)]
pub struct List {
    pub message_number: Option<usize>,
}

/// A message-number (required)
#[derive(Debug, PartialEq)]
pub struct Retr {
    pub message_number: usize,
}

/// A message-number (required)
#[derive(Debug, PartialEq)]
pub struct Dele {
    pub message_number: usize,
}

#[derive(Debug, PartialEq)]
pub struct Noop;

#[derive(Debug, PartialEq)]
pub struct Rset;

#[derive(Debug, PartialEq)]
pub struct Quit;

/// A message-number (required)
/// A non-negative number of lines (required)
#[derive(Debug, PartialEq)]
pub struct Top {
    pub message_number: usize,
    pub line_numnber: usize,
}

/// A message-number (optional)
#[derive(Debug, PartialEq)]
pub struct Uidl {
    pub message_number: Option<usize>,
}

/// A string identifying a mailbox (required)
#[derive(Debug, PartialEq)]
pub struct User<'a> {
    pub name: &'a [u8],
}