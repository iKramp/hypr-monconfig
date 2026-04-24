use clay_layout::{
    Clay, Declaration, fixed, grow, layout::LayoutDirection, renderers::clay_raylib_render,
};
use raylib::prelude::*;

use crate::ui::monitor_area;

mod ui;

fn main() {
    let mut clay = Clay::new((800.0, 600.0).into());

    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .resizable()
        .title("Clay Layout with Raylib")
        .build();

    let mut frame_count = 0;

    while !rl.window_should_close() {
        let begin = std::time::Instant::now();
        let width = rl.get_screen_width() as f32;
        let height = rl.get_screen_height() as f32;
        clay.set_layout_dimensions((width, height).into());

        let mut clay = clay.begin::<_, _>();

        let min_split = 0.0;
        let max_split = {
            let size = height;
            let divider_size = ui::DIVIDER_SIZE;
            1.0 - (divider_size / size)
        };

        let non_adjusted_split = f32::sin(frame_count as f32 * 0.01) * 0.5 + 0.5;
        let layout_settings_split = min_split + (max_split - min_split) * non_adjusted_split;

        ui::draw_ui(
            &mut clay,
            &ui::DrawUiInfo {
                layout_settings_split,
            },
        );

        let render_commands = clay.end();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        clay_raylib_render(&mut d, render_commands, |_, _| {});
        drop(d);

        frame_count += 1;

        //limit to 60 fps
        let elapsed = begin.elapsed();
        let target_frame_time = std::time::Duration::from_secs_f32(1.0 / 144.0);
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
    }
}
