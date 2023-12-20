use crate::common::StatusIndicator;

#[derive(Debug, PartialEq)]
pub struct Greeting {
    pub status_indicator: StatusIndicator,
    pub message: Vec<u8>,
}

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
/// +OK
/// the number of messages in the maildrop
/// the size of the maildrop in octets
#[derive(Debug, PartialEq)]
pub struct Stat {
    pub status_indicator: StatusIndicator,
    pub number_of_messages: usize,
    pub size_in_octets: usize,
    pub message: Vec<u8>,
}
