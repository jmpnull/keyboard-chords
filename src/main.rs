use std::io::{stdin, stdout, Write};
use std::time::Duration;

use keyboard_chords::{
    Chord, Press,
    VirtualKey::{Backspace, Enter},
};

#[tokio::main]
async fn main() {
    // Create a new chord
    let mut chord = Chord::new();

    // Pushing a string will append the required key presses
    chord.push_str("Hello, wro");

    // We can also push (multiple) virtual keys
    chord.push_n(Press::from(Backspace), 3);

    chord.push_str("World");

    // 'Typewriter' mode won't overwrite preset delays
    chord.push(Press::from('!' as u16).with_delay(Duration::from_millis(500)));

    // Add an enter-key at the end, so that our input will complete
    chord.push(Press::from(Enter));

    // Emulate typing delay!
    chord.typewriter(25..175);

    // Wait some time before playing the keys back
    tokio::task::spawn(chord.play_after(Duration::from_millis(500)));

    // Read the input, and check our result
    print!("Robot, please say hello: ");
    let _ = stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line!");

    let input = input.trim();

    assert_eq!(input, "Hello, World!");
}
