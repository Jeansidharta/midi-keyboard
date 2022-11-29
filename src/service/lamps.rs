use palette::{rgb::Rgb, Hsv, IntoColor, RgbHue};

fn rgb_to_number(rgb: Rgb) -> u32 {
    return ((rgb.red * 0xff as f32) as u32) * 0x10000
        + ((rgb.green * 0xff as f32) as u32) * 0x100
        + ((rgb.blue * 0xff as f32) as u32);
}

pub fn set_scene_color(targets: String, hue: f32, saturation: f32, brightness: u8) -> String {
    let rgb: Rgb = Hsv::new(RgbHue::from_degrees(hue), saturation, 1.0).into_color();

    return format!(
        r#"{{
            "type": "call-lamp-method",
            "data": {{
                "args": ["color", {}, {}],
                "targets": [{}],
                "method": "set_scene"
            }}
        }}"#,
        rgb_to_number(rgb),
        brightness,
        targets
    );
}

pub fn set_scene_color_temperature(
    targets: String,
    color_temperature: u32,
    brightness: u8,
) -> String {
    return format!(
        r#"{{
            "type": "call-lamp-method",
            "data": {{
                "args": ["ct", {}, {}],
                "targets": [{}],
                "method": "set_scene"
            }}
        }}"#,
        color_temperature, brightness, targets
    );
}

pub fn toggle_lamp(targets: String) -> String {
    return format!(
        r#"{{
            "type": "call-lamp-method",
            "data": {{
                "args": [],
                "targets": [{}],
                "method": "toggle"
            }}
        }}"#,
        targets
    );
}

pub fn blink_lamp_green(targets: String) -> String {
    let green = rgb_to_number(Hsv::new(RgbHue::from_degrees(120.0), 100.0, 1.0).into_color());

    println!("targets: {}", targets);

    return format!(
        r#"{{
            "type": "call-lamp-method",
            "data": {{
                "args": [1, 0, "100, 1, {}, 100, 100, 1, {}, 100"],
                "targets": [{}],
                "method": "start_cf"
            }}
        }}"#,
        green, 0, targets
    );
}

pub fn blink_lamp_red(targets: String) -> String {
    let red = rgb_to_number(Hsv::new(RgbHue::from_degrees(0.0), 100.0, 1.0).into_color());

    println!("targets: {}", targets);

    return format!(
        r#"{{
            "type": "call-lamp-method",
            "data": {{
                "args": [1, 0, "100, 1, {}, 100, 100, 1, {}, 100"],
                "targets": [{}],
                "method": "start_cf"
            }}
        }}"#,
        red, 0, targets
    );
}
