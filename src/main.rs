use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use std::rc::Rc;
use std::time::Instant;

struct AppData {
    counter: usize,
    frame_count: u32,
    last_update: Instant,
    fps: f32,
    draw_time: f32,
    font: Font,
}

fn prep_label(font: &Font, txt: &str) -> Rc<speedy2d::font::FormattedTextBlock> {
    font.layout_text(txt, 42.0, TextOptions::new())
}

fn main() {
    // let window = Window::new_centered("Speedy2D: Hello World", (640, 240)).unwrap();
    let window = Window::new_fullscreen_borderless("FPS draw").unwrap();
    window.run_loop(AppData {
        counter: 0,
        frame_count: 0,
        last_update: std::time::Instant::now(),
        font: Font::new(include_bytes!(
            "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf"
        ))
        .unwrap(),
        fps: 0.0,
        draw_time: 0.0,
    })
}

impl WindowHandler for AppData {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(speedy2d::color::Color::BLACK);
        let draw_start = Instant::now();
        let text = prep_label(
            &self.font,
            &std::format!(
                "Frame: {}, FPS: {:.2}, frame draw time: {:.2} µs",
                self.counter,
                self.fps,
                self.draw_time
            ),
        );
        graphics.draw_text((0.0, 0.0), speedy2d::color::Color::RED, &text);
        helper.request_redraw();
        self.draw_time = draw_start.elapsed().as_secs_f32() * 1000.0 * 1000.0;
        self.counter += 1;
        self.frame_count += 1;
        self.fps = 1.0 / (draw_start - self.last_update).as_secs_f32();
        self.last_update = draw_start;
    }
}
