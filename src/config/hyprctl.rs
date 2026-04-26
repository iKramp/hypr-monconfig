pub fn parse_hyprctl() -> crate::ui::monitor_area::MonitorLocationData {
    let output = std::process::Command::new("hyprctl")
        .arg("monitors")
        .output()
        .expect("Failed to execute hyprctl command");

    let mut monitor_data = crate::ui::monitor_area::MonitorLocationData {
        dragging: false,
        scale: 0.1,
        position: (0., 0.),
        monitors: std::collections::HashMap::new(),
    };

    let content = String::from_utf8_lossy(&output.stdout).to_string();
    let mut lines = content.lines();
    'main_loop: while let Some(mut line) = lines.next() {
        while !line.contains("Monitor") {
            let Some(next_line) = lines.next() else {
                break 'main_loop;
            };
            line = next_line;
        }

        let name = line.split_whitespace().nth(1).unwrap_or("unknown monitor");
        println!("Monitor name: {}", name);

        let dimensions_line = lines.next().unwrap_or("100x100@60");
        let main_dimensions_part = dimensions_line.split_whitespace().next().unwrap_or("");
        let (dimensions, framerate) = main_dimensions_part
            .split_once('@')
            .unwrap_or((main_dimensions_part, "0"));
        println!("Dimensions: {}, Framerate: {}", dimensions, framerate);

        let main_position_part = dimensions_line.split_whitespace().nth(2).unwrap_or("0x0");
        println!("Position: {}", main_position_part);

        let physical_line = loop {
            let Some(next_line) = lines.next() else {
                break 'main_loop;
            };
            if next_line.contains("physical size") {
                break next_line;
            }
        };
        let physical_main_part = physical_line.split_whitespace().nth(3).unwrap_or("0x0");
        println!("Physical size: {}", physical_main_part);

        let rotation_line = loop {
            let Some(next_line) = lines.next() else {
                break 'main_loop;
            };
            if next_line.contains("transform") {
                break next_line;
            }
        };
        let rotation_part = rotation_line.split_whitespace().nth(1).unwrap_or("0");

        let (size_px_x, size_px_y) = physical_main_part
            .split_once('x')
            .map(|(x, y)| (x.parse::<u32>().unwrap_or(0), y.parse::<u32>().unwrap_or(0)))
            .unwrap_or((0, 0));
        let (size_x, size_y) = dimensions
            .split_once('x')
            .map(|(x, y)| {
                (
                    x.parse::<f32>().unwrap_or(0.),
                    y.parse::<f32>().unwrap_or(0.),
                )
            })
            .unwrap_or((0., 0.));
        let (position_x, position_y) = main_position_part
            .split_once('x')
            .map(|(x, y)| {
                (
                    x.parse::<f32>().unwrap_or(0.),
                    y.parse::<f32>().unwrap_or(0.),
                )
            })
            .unwrap_or((0., 0.));
        let rotation = rotation_part.parse::<u32>().unwrap_or(0);

        monitor_data.monitors.insert(
            name.into(),
            crate::ui::monitor_area::MonitorInfo {
                name: name.into(),
                position: (position_x, position_y),
                size: (size_x, size_y),
                size_px: (size_px_x, size_px_y),
                rotation,
            },
        );
    }

    monitor_data
}
