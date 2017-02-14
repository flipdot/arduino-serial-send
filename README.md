# arduino-serial-send

## Building

On raspberry, requires rust to be installed:

```sh
cargo build
```

On x86_64, requires [rustup][]:

* Install a C cross compiler for linking C deps
  * E.g. on arch: arm-linux-gnueabihf-gcc from AUR
* Put the following into your `~/.cargo/config`:
  
  ```toml
  [target.arm-unknown-linux-gnueabihf]
  linker = "arm-linux-gnueabihf-gcc"
  ```

  (or whatever your arm cross compiler executable is called)

* `rustup target add arm-unknown-linux-gnueabihf`
* `cargo build --target arm-unknown-linux-gnueabihf`

[rustup]: https://rustup.rs/

## Example Usage

```sh
mkfifo /tmp/send_to_arduino
nohup ./arduino-serial-send --serial /dev/ttyUSB0 &
```

## Troubleshooting

**I started it but it doesn't do anything!**

For some reason, the fifo sometimes seems to be created with root as the owner and group.
Try killing arduino-serial-send, recreating the fifo or updating the owner / group and
then starting it again.

## ToDo / Ideas / Roadmap

* Daemonize by itself
  * Unclear how to log errors
* Add example autostart configuration
  * Autostart with the system? With device connection? Which privileges to drop to?
* Use a different input method
  * Which one?
    * DBus: Too complex / pointless dependency?
    * Socket file: TCP? UDP? How to send things from a script?
  * Alternative: Find a way to create the FIFO with Rust
    * Can we read new data from it after EOF without reopening it?
