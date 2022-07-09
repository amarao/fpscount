# fpscount

FPS count is a simple X11/Wayland utility which shows frame count, FPS and draw time for each frame.

It has two primary objectives:
* To allow speedcam screen recording to check if monitor is skip frames or delay output.
* To practice with speedy2d library (which is doing most heavylifting here).

How to use:

1. Download binary (under construction)
2. Build from source

## Donwloading binary
(not yet)

## Building from source

1. Clone this repo:
   ```sh
   git clone https://github.com/amarao/fpscount
   ```

2. Compile
  ```sh
  cargo build --release
  ```

3. Run:
   ```sh
   ./target/release/fpscount
   ```

4. Press Alt-F4 to exit.

