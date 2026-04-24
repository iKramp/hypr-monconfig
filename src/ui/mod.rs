use clay_layout::{
    ClayLayoutScope, Declaration,
    elements::FloatingAttachPointType,
    fixed, grow,
    layout::{LayoutDirection, Sizing},
    percent,
};
use raylib::texture::Texture2D;

pub mod monitor_area;

pub const DIVIDER_SIZE: f32 = 16.;

pub struct DrawUiInfo {
    pub layout_settings_split: f32,
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
            let (monitor_part, settings_part) = section_main_area::<'b>(info.layout_settings_split);
            c.with(&monitor_part, |c| {
                c.with(&monitor_area::draw_monitor_area(1.0, (0., 0.)), |_c| {
                    // Draw monitor area content here
                });
            });
            c.with(
                Declaration::new()
                    .layout()
                        .width(grow!())
                        .height(fixed!(DIVIDER_SIZE))
                    .end()
                    .background_color((252, 186, 3).into()),
                |_c| {
                    // Draw divider here (optional)
                },
            );
            c.with(&settings_part, |_c| {
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
        .end()
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
