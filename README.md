# fpscount

FPS count is a simple X11/Wayland utility which shows frame count, FPS and draw time for each frame.

![image](https://user-images.githubusercontent.com/652496/178106067-ab6385cf-5243-4627-9335-9f56b87b6f31.png)

It has two primary objectives:
* To allow speedcam screen recording to check if monitor is skip frames or delay output.
* To practice with speedy2d library (which is doing most heavylifting here).

How to use:

1. Download binary (under construction)
2. Build from source

## Donwloading binary
(not yet)

## Building from source

(instruction for Linux)

1. Install `fonts-noto-core` package. Build process expects to find
   `/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf` font file.

2. Clone this repo:
   ```sh
   git clone https://github.com/amarao/fpscount
   ```

3. Compile
  ```sh
  cargo build --release
  ```

4. Run:
   ```sh
   ./target/release/fpscount
   ```

5. Press Alt-F4 to exit.

