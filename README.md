keylogger-rs

listen to keyboard event, and save them to `/tmp/key.log`.

usage:

cargo build --release

sudo ./target/release/skynet &

watch -n 1 sudo cat /tmp/key.log

