use std::collections::HashMap;

use clay_layout::{
    ClayLayoutScope, Declaration,
    elements::{FloatingAttachPointType, FloatingAttachToElement},
    fixed,
    math::Vector2,
};
use raylib::texture::Texture2D;

pub struct MonitorInfo {
    pub name: Box<str>,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub size_px: (u32, u32),
    pub rotation: u32,
}

pub struct MonitorLocationData {
    pub dragging: bool,
    pub scale: f32,
    pub position: (f32, f32),
    pub monitors: HashMap<Box<str>, MonitorInfo>,
}

#[rustfmt::skip]
pub fn draw_monitor_area<'a, 'b>(data: &MonitorLocationData, layout_scope: &mut ClayLayoutScope<'a, 'b, Texture2D, ()>)
where
    'a: 'b,
{

    for monitor in data.monitors.values() {
        let mut monitor_declaration = Declaration::new();

        let top_left = (
            monitor.position.0 * data.scale,
            monitor.position.1 * data.scale,
        );
        let size = if monitor.rotation % 2 == 0 {(
            monitor.size.0 * data.scale,
            monitor.size.1 * data.scale,
        )} else {(
            monitor.size.1 * data.scale,
            monitor.size.0 * data.scale,
        )};

        let color = match monitor.rotation {
            0 => (0x50, 0x50, 0x50),
            1 => (0x50, 0x50, 0x80),
            2 => (0x50, 0x80, 0x50),
            3 => (0x80, 0x50, 0x50),
            _ => (0x50, 0x50, 0x50),
        };
        

        monitor_declaration
            .floating()
                .offset(top_left.into())
                .attach_to(FloatingAttachToElement::Parent)
                .attach_points(FloatingAttachPointType::LeftTop, FloatingAttachPointType::CenterCenter)
            .end()
            .layout()
                .width(fixed!(size.0))
                .height(fixed!(size.1))
            .end()
            .corner_radius()
                .all(12.)
            .end()
            .background_color(color.into());
        layout_scope.with(&monitor_declaration, |_| {});
    }
}
