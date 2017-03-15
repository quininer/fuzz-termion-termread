#![no_main]
extern crate libfuzzer_sys;
extern crate termion;

use std::io::{ self, Read, Write, Cursor };
use termion::input::TermRead;
use termion::event::Key;


fn echo_char<I, O>(input: I, mut output: O) -> io::Result<()>
    where I: Read, O: Write
{
    for key in input.keys() {
        if let Ok(Key::Char(c)) = key {
            write!(output, "{}", c)?;
        }
    }

    Ok(())
}

#[export_name="rust_fuzzer_test_input"]
pub extern fn go(data: &[u8]) {
    let input = Cursor::new(data);

    let _ = echo_char(input, io::sink());
}
