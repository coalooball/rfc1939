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
