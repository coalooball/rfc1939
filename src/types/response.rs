use crate::common::StatusIndicator;

/// Greeting
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct Greeting<'a> {
    pub status_indicator: StatusIndicator,
    pub message: &'a [u8],
}

/// Quit
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct Quit<'a> {
    pub status_indicator: StatusIndicator,
    pub message: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct OneLineTwoParts<'a> {
    pub left: StatusIndicator,
    pub right: &'a [u8],
}

impl Default for OneLineTwoParts<'_> {
    fn default() -> Self {
        OneLineTwoParts {
            left: StatusIndicator::OK,
            right: &[],
        }
    }
}

impl Default for Quit<'_> {
    fn default() -> Self {
        Quit {
            status_indicator: StatusIndicator::OK,
            message: &[],
        }
    }
}

impl Default for Greeting<'_> {
    fn default() -> Self {
        Greeting {
            status_indicator: StatusIndicator::OK,
            message: &[],
        }
    }
}

pub trait OneLine<'a>: Default {
    fn status_indicator(&self) -> &StatusIndicator;
    fn set_status_indicator(&mut self, si: StatusIndicator);

    fn message(&self) -> &[u8];
    fn set_message(&mut self, message: &'a [u8]);
}

impl<'a> OneLine<'a> for Greeting<'a> {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.status_indicator
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }

    fn message(&self) -> &[u8] {
        &self.message
    }

    fn set_message(&mut self, message: &'a [u8]) {
        self.message = message;
    }
}

impl<'a> OneLine<'a> for Quit<'a> {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.status_indicator
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }

    fn message(&self) -> &[u8] {
        &self.message
    }

    fn set_message(&mut self, message: &'a [u8]) {
        self.message = message;
    }
}

impl<'a> OneLine<'a> for OneLineTwoParts<'a> {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.left
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.left = si;
    }

    fn message(&self) -> &[u8] {
        &self.right
    }

    fn set_message(&mut self, message: &'a [u8]) {
        self.right = message;
    }
}

/// STAT
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// usize, the number of messages in the maildrop
/// usize, the size of the maildrop in octets
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct Stat<'a> {
    pub status_indicator: StatusIndicator,
    pub number_of_messages: usize,
    pub size_in_octets: usize,
    pub message: &'a [u8],
}

// -- Code Block in which item have a list contains paired informations.
// ################################################################################

/// LIST [msg]
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// Vec<(usize, usize)>, A vector containing tuple in which
///     left usize is message-number and
///     right usize is size of the message in octets
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct List<'a> {
    pub status_indicator: StatusIndicator,
    pub informations: Vec<(usize, usize)>,
    pub message: &'a [u8],
}

/// UIDL [msg]
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// Vec<(usize, usize)>, A vector containing tuple in which
///     left usize is message-number and
///     right usize is size of the message in octets
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct Uidl<'a> {
    pub status_indicator: StatusIndicator,
    pub informations: Vec<(usize, &'a [u8])>,
    pub message: &'a [u8],
}

// ################################################################################
// -- Code Block in which item have a list contains paired informations. --

// -- Code Block in which item have email body.
// ################################################################################
// Both RETR and TOP may have a email body.
pub trait HaveMessageBody<'a>: Default {
    fn set_status_indicator(&mut self, si: StatusIndicator);
    fn set_message_body(&mut self, body: Option<&'a [u8]>);
    fn set_message(&mut self, msg: &'a [u8]);
}
/// RETR msg
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// &[u8], A slice containing message body
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct Retr<'a> {
    pub status_indicator: StatusIndicator,
    pub message_body: Option<&'a [u8]>,
    pub message: &'a [u8],
}

impl Default for Retr<'_> {
    fn default() -> Self {
        Retr {
            status_indicator: StatusIndicator::OK,
            message_body: None,
            message: &[],
        }
    }
}

impl<'a> HaveMessageBody<'a> for Retr<'a> {
    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }
    fn set_message(&mut self, msg: &'a [u8]) {
        self.message = msg;
    }
    fn set_message_body(&mut self, body: Option<&'a [u8]>) {
        self.message_body = body;
    }
}

/// TOP msg n
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// &[u8], A slice containing message body
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct Top<'a> {
    pub status_indicator: StatusIndicator,
    pub message_body: Option<&'a [u8]>,
    pub message: &'a [u8],
}

impl Default for Top<'_> {
    fn default() -> Self {
        Top {
            status_indicator: StatusIndicator::OK,
            message_body: None,
            message: &[],
        }
    }
}

impl<'a> HaveMessageBody<'a> for Top<'a> {
    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }
    fn set_message(&mut self, msg: &'a [u8]) {
        self.message = msg;
    }
    fn set_message_body(&mut self, body: Option<&'a [u8]>) {
        self.message_body = body;
    }
}

// ################################################################################
// -- Code Block in which item have email body. --

/// DELE msg
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct Dele<'a> {
    pub status_indicator: StatusIndicator,
    pub message: &'a [u8],
}

impl Default for Dele<'_> {
    fn default() -> Self {
        Dele {
            status_indicator: StatusIndicator::OK,
            message: &[],
        }
    }
}

impl<'a> OneLine<'a> for Dele<'a> {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.status_indicator
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }

    fn message(&self) -> &[u8] {
        &self.message
    }

    fn set_message(&mut self, message: &'a [u8]) {
        self.message = message;
    }
}

/// NOOP
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct Noop<'a> {
    pub status_indicator: StatusIndicator,
    pub message: &'a [u8],
}

impl Default for Noop<'_> {
    fn default() -> Self {
        Noop {
            status_indicator: StatusIndicator::OK,
            message: &[],
        }
    }
}

impl<'a> OneLine<'a> for Noop<'a> {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.status_indicator
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }

    fn message(&self) -> &[u8] {
        &self.message
    }

    fn set_message(&mut self, message: &'a [u8]) {
        self.message = message;
    }
}

/// RSET
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct Rset<'a> {
    pub status_indicator: StatusIndicator,
    pub message: &'a [u8],
}

impl Default for Rset<'_> {
    fn default() -> Self {
        Rset {
            status_indicator: StatusIndicator::OK,
            message: &[],
        }
    }
}

impl<'a> OneLine<'a> for Rset<'a> {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.status_indicator
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }

    fn message(&self) -> &[u8] {
        &self.message
    }

    fn set_message(&mut self, message: &'a [u8]) {
        self.message = message;
    }
}

/// USER
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct User<'a> {
    pub status_indicator: StatusIndicator,
    pub message: &'a [u8],
}

impl Default for User<'_> {
    fn default() -> Self {
        User {
            status_indicator: StatusIndicator::OK,
            message: &[],
        }
    }
}

impl<'a> OneLine<'a> for User<'a> {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.status_indicator
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }

    fn message(&self) -> &[u8] {
        &self.message
    }

    fn set_message(&mut self, message: &'a [u8]) {
        self.message = message;
    }
}

/// PASS
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// &[u8], message
#[derive(Debug, PartialEq)]
pub struct Pass<'a> {
    pub status_indicator: StatusIndicator,
    pub message: &'a [u8],
}

impl Default for Pass<'_> {
    fn default() -> Self {
        Pass {
            status_indicator: StatusIndicator::OK,
            message: &[],
        }
    }
}

impl<'a> OneLine<'a> for Pass<'a> {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.status_indicator
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }

    fn message(&self) -> &[u8] {
        &self.message
    }

    fn set_message(&mut self, message: &'a [u8]) {
        self.message = message;
    }
}
