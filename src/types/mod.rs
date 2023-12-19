#[derive(Debug, PartialEq)]
pub enum StatusIndicator {
    OK,
    ERR,
}

#[derive(Debug, PartialEq)]
pub struct Greeting {
    pub status_indicator: StatusIndicator,
    pub message: Vec<u8>,
}
