use clay_layout::{
    Clay_GetScrollOffset, ClayLayoutScope, Declaration, fixed, grow,
    layout::{Alignment, LayoutAlignmentX, LayoutAlignmentY, LayoutDirection, Sizing},
};
use raylib::{RaylibHandle, ffi::MouseButton, texture::Texture2D};

pub mod monitor_area;

pub const DIVIDER_SIZE: f32 = 8.;

pub struct DrawUiInfo {
    pub layout_settings_split: f32,
    pub size_px: (f32, f32),
    pub dragging_divider: bool,
    pub monitor_location_data: monitor_area::MonitorLocationData,
}

pub fn draw_ui<'a, 'b>(layout_scope: &mut ClayLayoutScope<'a, 'b, Texture2D, ()>, info: &DrawUiInfo)
where
    'a: 'b,
{
    #[rustfmt::skip]
    layout_scope.with(Declaration::new()
        .layout()
            .width(grow!())
            .height(grow!())
            .direction(LayoutDirection::TopToBottom)
        .end(), |c| {
            let (mut monitor_part, mut settings_part) = section_main_area::<'b>(info.layout_settings_split);

            c.with(monitor_part.id(c.id("monitor_layout_area")), |c| {
                monitor_area::draw_monitor_area(&info.monitor_location_data, c);
            });
            c.with(
                Declaration::new()
                    .layout()
                        .width(grow!())
                        .height(fixed!(DIVIDER_SIZE))
                    .end()
                    .id(c.id("divider"))
                    .background_color((252, 186, 3).into()),
                |_c| {
                    // Draw divider here (optional)
                },
            );
            c.with(settings_part.id(c.id("monitor_settings_area")), |_c| {
                // Draw settings area content here
            });
        });
}

#[rustfmt::skip]
pub fn section_main_area<'a>(percentage: f32) -> (Declaration<'a, Texture2D, ()>, Declaration<'a, Texture2D, ()>) {
    

    let mut monitor_declaration = Declaration::new();
    monitor_declaration
        .layout()
            .width(grow!())
            .height(Sizing::Percent(percentage))
            .child_alignment(Alignment::new(LayoutAlignmentX::Center, LayoutAlignmentY::Center))
        .end()
        .clip(true, true, unsafe { Clay_GetScrollOffset().into() })
        .corner_radius()
            .all(24.)
        .end()
        .background_color((90, 85, 181).into()); //blue

    let mut settings_declaration = Declaration::new();
    settings_declaration
        .layout()
            .width(grow!())
            .height(grow!())
        .end()
        .corner_radius()
            .all(24.)
        .end()
        .background_color((85, 181, 128).into()); //green
    (monitor_declaration, settings_declaration)
}

pub fn drag_divider<'a, 'b>(
    rl: &mut RaylibHandle,
    layout: &mut ClayLayoutScope<'a, 'b, Texture2D, ()>,
    draw_info: &mut DrawUiInfo,
) where
    'a: 'b,
{
    let Some(bounding_box) = layout.bounding_box(layout.id("divider")) else {
        draw_info.dragging_divider = false;
        return;
    };

    let bound_to_divider = draw_info.dragging_divider || {
        let mouse_position = rl.get_mouse_position();
        if mouse_position.x < bounding_box.x
            || mouse_position.x > bounding_box.x + bounding_box.width
            || mouse_position.y < bounding_box.y
            || mouse_position.y > bounding_box.y + bounding_box.height
        {
            // Mouse is outside the divider area, do not start dragging
            return;
        }
        true
    };

    let mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
    let drag = (draw_info.dragging_divider && mouse_down) || (bound_to_divider && mouse_pressed);

    if !drag {
        draw_info.dragging_divider = false;
        // Mouse button is not pressed, do not start dragging
        return;
    }

    draw_info.dragging_divider = true;

    let delta = rl.get_mouse_delta().y;
    let delta_percent = delta / draw_info.size_px.1;
    draw_info.layout_settings_split += delta_percent;

    let min_split = 0.0;

    let total_divider_percent = DIVIDER_SIZE / draw_info.size_px.1;
    let max_split = 1.0 - total_divider_percent;

    draw_info.layout_settings_split = draw_info.layout_settings_split.clamp(min_split, max_split);
}
