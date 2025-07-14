use crate::VirtualKey;
use std::time::Duration;

/// Keyboard events can be unicode-renderable characters (like 'A'), but can
/// also take the form of virutal key events (like 'ESC').
///
/// This enum encapsulates the two, in order to allow the underlying driver
/// to diffrentiate between otherwise identical codes.
#[derive(Debug, Clone, PartialEq)]
pub enum Code {
    VirtualKey(u16),
    UnicodeKey(u16),
}

/// Keypress events are virtual or unicode key events, over some duration
///
/// If duration is `None` the press will be insterted into the input stream as a keydown
/// immediately followed by a keyup.
#[derive(Debug, Clone, PartialEq)]
pub struct Press {
    /// The virtual or unicode key event
    pub code: Code,

    /// The delay to wait before sending key-down
    pub delay: Option<Duration>,

    /// The duration to hold the key down for
    pub duration: Option<Duration>,
}

impl Press {
    /// Create a new keypress event; by default this
    /// will be a 'unicode' style keypress
    pub fn new(code: impl Into<u16>) -> Self {
        Self {
            code: Code::UnicodeKey(code.into()),
            delay: None,
            duration: None,
        }
    }

    /// Converts the press into a unicode style keypress
    pub fn as_unicode(mut self) -> Self {
        match self.code {
            Code::UnicodeKey(_) => {}
            Code::VirtualKey(k) => self.code = Code::UnicodeKey(k),
        }
        self
    }

    /// Converts the press into a virtual style keypress
    pub fn as_virtual(mut self) -> Self {
        match self.code {
            Code::UnicodeKey(k) => self.code = Code::VirtualKey(k),
            Code::VirtualKey(_) => {}
        }
        self
    }

    /// Adds a delay before key-down is transmitted for this press
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Adds a delay before key-up is transmitted for this press
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }
}

/// Helper used when generating key presses from utf16-encoded strings
impl<T: Into<u16>> From<T> for Press {
    fn from(code: T) -> Self {
        Self::new(code)
    }
}

/// Helper used when generating key presses from virtual key-codes
impl From<VirtualKey> for Press {
    fn from(code: VirtualKey) -> Self {
        Self::new(code as u16).as_virtual()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypress_from_utf16() {
        let expected = Press::new(0x0061_u16).as_unicode();
        let unicode_a = "a".encode_utf16().next().unwrap();

        let actual = Press::from(unicode_a);

        assert_eq!(actual, expected);
    }

    #[test]
    fn keypress_from_vk() {
        let expected = Press::new(0x0D_u16).as_virtual();
        let actual = Press::from(VirtualKey::Enter);

        assert_eq!(actual, expected);
    }
}
