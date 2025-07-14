//! `keyboard-chords`: A windows keyboard event library
//!
//! A `Chord` is a set of `key::Press` events that will be sent to the system.
//! Each key press event consists of sending a unicode or virutal KEY_DOWN event,
//! waiting for the specified press duration, and then sending KEY_UP.
//!
//! Notably, `keyboard-chords` can send the complete 'chord' in single a system `_send_inputs` call.
//! This allows you to, for instance, send a chord that presses the 'UP' and 'RIGHT' arrow keys, but holds the 'UP'
//! key for 3 seconds while releasing the 'RIGHT' key after only 500 ms.
//!
//! The primary focus of `keyboard-chords` is to  make it as simple as possible to simulate
//! keyboard inputs. For instance, consider this `Chord` that sends 'Hello, world!':
//!
//! ```no_run
//! # use std::io::{stdin, stdout, Write};
//! # use std::time::Duration;
//! #
//! use keyboard_chords::Chord;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a new chord
//!     let mut chord = Chord::new();
//!
//!     // Pushing a string will append the required key presses
//!     chord.push_str("Hello, world!");
//!
//!     // Emulate typing delay of 25 to 175ms per keypress
//!     chord.typewriter(25..175);
//!
//!     // Wait some time before playing the keys back
//!     chord.play_after(Duration::from_millis(500)).await;
//! }
//! ```

/// Support sending input events on windows platform, via the `SendInput` API
#[cfg(target_os = "windows")]
mod win;

/// Provides a `Press` type, that respresents pressing a key for some duration.
///
/// `Press` events are used to a sequence of key-down + key-up events when playing
/// a `Chord` of keypresses.
pub mod key;
pub use key::Press;

/// Provides the list of virtual key codes on windows/unix machines
pub mod codes;
pub use codes::VirtualKey;

use rand::Rng;

/// A `Chord` is a group of key-presses that will be transmitted in-bulk to the system
#[derive(Debug)]
pub struct Chord {
    pub keys: Vec<Press>,
}

impl Default for Chord {
    fn default() -> Self {
        Self::new()
    }
}

impl Chord {
    /// Create a new chord
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    /// Append a keypress to the end of the chord
    pub fn push(&mut self, press: Press) {
        self.keys.push(press)
    }

    /// Append a keypress to the end of the chord
    pub fn push_n(&mut self, press: Press, count: usize) {
        for _ in 0..count {
            self.push(press.clone());
        }
    }

    /// Push the keypresses required to write the string to the end of the chord
    pub fn push_str(&mut self, keys: &str) {
        for k in keys.encode_utf16() {
            self.keys.push(Press::from(k))
        }
    }

    pub fn typewriter(&mut self, range: std::ops::Range<u64>) {
        let mut rng = rand::rng();

        for p in self.keys.iter_mut() {
            if p.delay.is_some() {
                continue;
            }

            p.delay = Some(std::time::Duration::from_millis(
                rng.random_range(range.clone()),
            ))
        }
    }

    /// Playback the key events after some delay
    pub async fn play_after(self, duration: std::time::Duration) {
        tokio::time::sleep(duration).await;
        self.play().await
    }

    /// Playback the key events to the system
    pub async fn play(&self) {
        #[cfg(target_os = "windows")]
        win::send_inputs(&self.keys).await
    }
}
