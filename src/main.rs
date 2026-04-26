use clay_layout::{Clay, renderers::clay_raylib_render};
use raylib::prelude::*;

use crate::ui::drag_divider;

mod config;
mod ui;

fn main() {
    let mut clay = Clay::new((800.0, 600.0).into());

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .resizable()
        .title("Clay Layout with Raylib")
        .build();

    let mut draw_info = ui::DrawUiInfo {
        layout_settings_split: 0.5,
        size_px: (800.0, 600.0),
        dragging_divider: false,
        monitor_location_data: config::hyprctl::parse_hyprctl(),
    };

    while !rl.window_should_close() {
        let begin = std::time::Instant::now();
        let width = rl.get_screen_width() as f32;
        let height = rl.get_screen_height() as f32;
        draw_info.size_px = (width, height);
        clay.set_layout_dimensions((width, height).into());

        let mut clay = clay.begin::<_, _>();

        ui::draw_ui(&mut clay, &draw_info);

        drag_divider(&mut rl, &mut clay, &mut draw_info);

        let render_commands = clay.end();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        clay_raylib_render(&mut d, render_commands, |_, _| {});
        drop(d);

        //limit to 60 fps
        let elapsed = begin.elapsed();
        let target_frame_time = std::time::Duration::from_secs_f32(1.0 / 144.0);
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
    }
}
