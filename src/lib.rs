use wasm_bindgen::prelude::*;
use image::{load_from_memory, DynamicImage, GenericImageView, Rgb};
use std::collections::HashMap;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "colors.pest"]
pub struct ColorParser;

fn hex_to_rgb(hex: &str) -> Rgb<u8> {
    let hex = hex.trim_start_matches("#");
    let hex_value = u32::from_str_radix(hex, 16).expect("Invalid hex value!");
    let red = (hex_value >> 16) as u8;
    let green = (hex_value >> 8 & 0xFF) as u8;
    let blue = (hex_value & 0xFF) as u8;
    Rgb([red, green, blue])
}

fn rgb_to_hex(rgb: &Rgb<u8>) -> String {
    let red = rgb[0];
    let green = rgb[1];
    let blue = rgb[2];
    format!("#{:02X}{:02X}{:02X}", red, green, blue)
}

fn get_nearest_color(pixel: &image::Rgba<u8>, color_map: &HashMap<Rgb<u8>, i32> ) -> i32 {


    // check if the pixel is transparent
    if pixel[3] == 0 {
        return 0;
    }

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

fn image_to_makecode_string(img: DynamicImage, color_map: &HashMap<Rgb<u8>, i32>) -> String {
    
    let img = img.to_rgba8();

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

    return img_string;
}


pub fn parse_colors(unparsed_colors: String) -> HashMap<String, HashMap<Rgb<u8>, i32>> {
        let color_file = ColorParser::parse(Rule::file, &unparsed_colors).expect("Failed to parse color file").next().unwrap();

    let mut color_maps: HashMap<String, HashMap<Rgb<u8>, i32>> = HashMap::new();

    for palette in color_file.into_inner() {
        let mut palette_name = String::new();
        let mut color_map = HashMap::new();
        for (i, r) in palette.into_inner().into_iter().enumerate() {
            match r.as_rule() {
                Rule::palette_name => {
                    palette_name = r.as_str().to_string();
                    // println!("{}", r.as_str())
                },
                Rule::hex_color => {
                    // println!("{}", r.as_str());
                    color_map.insert(hex_to_rgb(r.as_str()), i as i32);
                }
                _ => {}
            }
        }
        
        color_maps.insert(palette_name, color_map);
    }
    return color_maps
}

#[wasm_bindgen]
pub fn load_image(bytes: &[u8], unparsed_colors: String, colormap_name: String, width: u32, height: u32) -> String {
    let color_maps= parse_colors(unparsed_colors);
    let color_map = color_maps.get(&colormap_name).expect("Invalid colormap!");

    let mut img = load_from_memory(bytes).expect("failed to load image");

    img = img.resize(width, height, image::imageops::FilterType::Nearest);

    let img_string = image_to_makecode_string(img, color_map);
    return img_string;
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}