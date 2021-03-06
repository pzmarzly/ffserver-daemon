# Simple daemon made for `ffserver` livestreaming

Please adjust the code to your likings. See https://pzmarzly.pl/2018/03/17/raspberry-pi-livestreaming.html for usage details.

Behavior of this program could probably be replicated (and improved) by using systemd socket-activated services. https://youtu.be/S9YmaNuvw5U?t=18m38s http://manpages.org/systemd-socket-proxyd/8 https://sarata.com/manpages/systemd.socket.5.html

### Features:

- Allows to automatically shut down the server when no clients are connected
- Manages ffserver and ffmpeg instances
- Restarts ffmpeg on crash

### Usage:

1. Edit `src/main.rs` to your liking
   - `/home/pi/ffserver.conf` is default configuration file
   - 83 is default server port
   - 90 is default feed port (make sure it matches `ffserver.conf`
   - `/dev/video0` is default device
2. Compile with `cargo build --release` (`cargo` gets installed with Rust compiler - [download it here](https://www.rust-lang.org/en-US/install.html))
3. Add `target/release/ffserver-daemon` to your crontab:
   ```
   @reboot /path/to/ffserver-daemon
   ```

### How does it work?

It listens on specified port, when client connects, it breaks the connection and starts `ffserver`. Client has to reconnect. Then it waits until all client disconnect or certain amount of time has passed (by default 15 minutes).
