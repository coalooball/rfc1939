use crate::common::StatusIndicator;

/// Greeting
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// Vec<u8>, message
#[derive(Debug, PartialEq)]
pub struct Greeting {
    pub status_indicator: StatusIndicator,
    pub message: Vec<u8>,
}

/// Quit
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// Vec<u8>, message
#[derive(Debug, PartialEq)]
pub struct Quit {
    pub status_indicator: StatusIndicator,
    pub message: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct OneLineTwoParts {
    pub left: StatusIndicator,
    pub right: Vec<u8>,
}

impl Default for OneLineTwoParts {
    fn default() -> Self {
        OneLineTwoParts {
            left: StatusIndicator::OK,
            right: Vec::new(),
        }
    }
}

impl Default for Quit {
    fn default() -> Self {
        Quit {
            status_indicator: StatusIndicator::OK,
            message: Vec::new(),
        }
    }
}

impl Default for Greeting {
    fn default() -> Self {
        Greeting {
            status_indicator: StatusIndicator::OK,
            message: Vec::new(),
        }
    }
}

pub trait OneLine: Default {
    fn status_indicator(&self) -> &StatusIndicator;
    fn set_status_indicator(&mut self, si: StatusIndicator);

    fn message(&self) -> &[u8];
    fn set_message(&mut self, message: Vec<u8>);
}

impl OneLine for Greeting {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.status_indicator
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }

    fn message(&self) -> &[u8] {
        &self.message
    }

    fn set_message(&mut self, message: Vec<u8>) {
        self.message = message;
    }
}

impl OneLine for Quit {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.status_indicator
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.status_indicator = si;
    }

    fn message(&self) -> &[u8] {
        &self.message
    }

    fn set_message(&mut self, message: Vec<u8>) {
        self.message = message;
    }
}

impl OneLine for OneLineTwoParts {
    fn status_indicator(&self) -> &StatusIndicator {
        &self.left
    }

    fn set_status_indicator(&mut self, si: StatusIndicator) {
        self.left = si;
    }

    fn message(&self) -> &[u8] {
        &self.right
    }

    fn set_message(&mut self, message: Vec<u8>) {
        self.right = message;
    }
}

/// STAT
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// usize, the number of messages in the maildrop
/// usize, the size of the maildrop in octets
/// Vec<u8>, message
#[derive(Debug, PartialEq)]
pub struct Stat {
    pub status_indicator: StatusIndicator,
    pub number_of_messages: usize,
    pub size_in_octets: usize,
    pub message: Vec<u8>,
}

/// LIST [msg]
/// StatusIndicator, Status Indicator stand for +OK/-ERR
/// Vec<(usize, usize)>, A vector containing tuple in which
///     left usize is message-number and
///     right usize is size of the message in octets
/// Vec<u8>, message
#[derive(Debug, PartialEq)]
pub struct List {
    pub status_indicator: StatusIndicator,
    pub informations: Vec<(usize, usize)>,
    pub message: Vec<u8>,
}
