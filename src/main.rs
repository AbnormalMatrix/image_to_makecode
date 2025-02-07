use image::Rgb;
use std::collections::HashMap;
use std::fs;

fn hex_to_rgb(hex: &str) -> Rgb<u8> {
    let hex = hex.trim_start_matches("#");
    let hex_value = u32::from_str_radix(hex, 16).expect("Invalid hex value!");
    let red = (hex_value >> 16) as u8;
    let green = (hex_value >> 8 & 0xFF) as u8;
    let blue = (hex_value & 0xFF) as u8;
    Rgb([red, green, blue])
}

fn get_nearest_color(pixel: &Rgb<u8>, color_map: &HashMap<Rgb<u8>, i32> ) -> i32 {
    let mut lowest_distance = f64::INFINITY;

    let mut best_color = 0;

    for (key, value) in color_map.into_iter() {
        let r_diff = key[0] as f64 - pixel[0] as f64;
        let g_diff = key[1] as f64 - pixel[1] as f64;
        let b_diff = key[2] as f64 - pixel[2] as f64;

        let total = (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff).abs();
        
        if total < lowest_distance {
            
            lowest_distance = total;
            best_color = value.clone();
        }
    }
    
    return best_color;
}

fn main() {
    let mut color_map = HashMap::new();
    color_map.insert(hex_to_rgb("#FFFFFF"), 1);
    color_map.insert(hex_to_rgb("#FF2121"), 2);
    color_map.insert(hex_to_rgb("#FF93C4"), 3);
    color_map.insert(hex_to_rgb("#FF8135"), 4);
    color_map.insert(hex_to_rgb("#FFF609"), 5);
    color_map.insert(hex_to_rgb("#249CA3"), 6);
    color_map.insert(hex_to_rgb("#78DC52"), 7);
    color_map.insert(hex_to_rgb("#003FAD"), 8);
    color_map.insert(hex_to_rgb("#87F2FF"), 9);
    color_map.insert(hex_to_rgb("#8E2EC4"), 10);
    color_map.insert(hex_to_rgb("#A4839F"), 11);
    color_map.insert(hex_to_rgb("#5C406C"), 12);
    color_map.insert(hex_to_rgb("#E5CDC4"), 13);
    color_map.insert(hex_to_rgb("#91463D"), 14);
    color_map.insert(hex_to_rgb("#000000"), 15);


    let mut img = image::open("cat.jpg").unwrap();
    img = img.resize(160, 120, image::imageops::FilterType::Nearest);
    let img = img.to_rgb8();

    let mut img_string = "img`".to_string();

    for py in 0..img.height() {
        let mut x_line = "    ".to_string();
        for px in 0..img.width() {
            let best_color = get_nearest_color(img.get_pixel(px, py), &color_map);
            x_line += &format!("{:x}",best_color);
            x_line += " ";
        }
        x_line += &"\n".to_string();
        img_string += &x_line;
    }
    img_string += "`";
    fs::write("img.txt", img_string).unwrap();
}
