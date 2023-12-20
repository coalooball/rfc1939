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
