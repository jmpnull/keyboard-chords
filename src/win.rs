//! Input transmission layer for windows devices

use std::time::Duration;

use crate::key::{Code, Press};

use windows::Win32::UI::Input::KeyboardAndMouse as WinKb;
use windows::Win32::UI::Input::KeyboardAndMouse::{SendInput, INPUT, KEYBDINPUT};

/// Convert from a keypress into a Windows keyboard event
///
/// Wraps the already existing key-press data into the Windows
/// specific keypress C FFI
impl From<&Press> for KEYBDINPUT {
    fn from(press: &Press) -> Self {
        #[allow(non_snake_case)]
        let (dwFlags, wVk, wScan) = match press.code {
            Code::VirtualKey(code) => (WinKb::KEYBD_EVENT_FLAGS(0), WinKb::VIRTUAL_KEY(code), code),
            Code::UnicodeKey(code) => (WinKb::KEYEVENTF_UNICODE, WinKb::VIRTUAL_KEY(0), code),
        };

        KEYBDINPUT {
            wVk,
            wScan,
            dwFlags,
            time: 0,
            dwExtraInfo: 0,
        }
    }
}

/// Converts a press into to an (Down, Up) input tuple.
///
/// Input events can also support non-keyboard sources, so this
/// wraps a keypress in the additional struct. It also provides
/// a struct for both the downstroke and upstroke: The windows `INPUT`
/// struct wraps a union, which cannot be modified by safe code.
impl From<&Press> for (INPUT, INPUT) {
    fn from(press: &Press) -> Self {
        let mut ki = press.into();

        let key_down = INPUT {
            r#type: WinKb::INPUT_KEYBOARD,
            Anonymous: WinKb::INPUT_0 { ki },
        };

        ki.dwFlags |= WinKb::KEYEVENTF_KEYUP;

        let key_up = INPUT {
            r#type: WinKb::INPUT_KEYBOARD,
            Anonymous: WinKb::INPUT_0 { ki },
        };

        (key_down, key_up)
    }
}

/// Helper function to send a single input event
async fn _send_input(input: INPUT, duration: Option<Duration>) {
    _send_inputs(&[input], duration).await
}

/// Sends an array of input events, optionally delaying prior to transmission
async fn _send_inputs(inputs: &[INPUT], duration: Option<Duration>) {
    if let Some(delay) = duration {
        tokio::time::sleep(delay).await;
    }

    unsafe {
        SendInput(inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

/// Consumes a list of `Press` events, and sends them to the system.
///
/// By default, press events are a 'keydown' immediately followed by a 'keyup'.
/// However, events with a delayed 'keyup' are also supported, allowing emulation
/// of a system key being held down.
pub async fn send_inputs(keys: &[Press]) {
    let mut inputs = Vec::new();
    let mut delays = Vec::new();

    for key in keys {
        let (down, up): (INPUT, INPUT) = key.into();

        // collect inputs until we get a delay
        if let Some(delay) = key.delay {
            // send inputs up to this point
            // Send the 'instantaneous' input events
            _send_inputs(&inputs, None).await;

            // Wait and send the following KEY_UP events
            futures::future::join_all(delays).await;

            // wait
            tokio::time::sleep(delay).await;

            // clear inputs/delays and keep going
            inputs.clear();
            delays = Vec::new();
        }

        inputs.push(down);

        if key.duration.is_none() {
            inputs.push(up);
        } else {
            // let up = &[up];
            delays.push(_send_input(up, key.duration));
        }
    }

    // Send the 'instantaneous' input events
    _send_inputs(&inputs, None).await;

    // Wait and send the following KEY_UP events
    futures::future::join_all(delays).await;
}
