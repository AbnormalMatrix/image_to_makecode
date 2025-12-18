use log::info;
use wasm_bindgen::prelude::*;
use image::{AnimationDecoder, DynamicImage, GenericImageView, ImageEncoder, Pixel, Rgb, Rgba, codecs::{gif::GifDecoder, png}, guess_format, load_from_memory};
use std::collections::HashMap;
use pest::Parser;
use pest_derive::Parser;
use palette::{color_difference::{Ciede2000, HyAb}, IntoColor, Lch, Oklab, Srgb};
use js_sys::{Uint8Array, Object, Reflect};
use std::io::Cursor;

mod median_cut;

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

struct BestColor {
    color_value: Rgba<u8>,
    color_number: i32,
}
impl BestColor {
    fn new(color_value: Rgba<u8>, color_number: i32) -> Self {
        Self { color_value, color_number }
    }
}

fn get_nearest_color(pixel: &image::Rgba<u8>, color_map: &HashMap<Rgb<u8>, i32>, check_transparency: bool ) -> BestColor {


    // check if the pixel is transparent
    if check_transparency && pixel[3] == 0 {
        return BestColor::new(pixel.clone(), 0);
    }


    let mut lowest_distance = f64::INFINITY;

    let mut best_color = BestColor::new(Rgba([0, 0, 0, 255]), 0);

    for (key, value) in color_map.into_iter() {
        let r_diff = key[0] as f64 - pixel[0] as f64;
        let g_diff = key[1] as f64 - pixel[1] as f64;
        let b_diff = key[2] as f64 - pixel[2] as f64;

        let total = (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff).abs();
        
        if total < lowest_distance {
            
            lowest_distance = total;
            // best_color = value.clone();
            best_color = BestColor::new(key.clone().to_rgba(), value.clone());
        }
    }
    
    return best_color;
}

fn get_nearest_color_hyab(pixel: &image::Rgba<u8>, color_map: &HashMap<Rgb<u8>, i32>, check_transparency: bool ) -> BestColor {


    // check if the pixel is transparent
    if check_transparency && pixel[3] == 0 {
        return BestColor::new(pixel.clone(), 0);
    }

    let pixel_color = Srgb::new(pixel[0], pixel[1], pixel[2]);

    let mut lowest_distance = f32::INFINITY;

    let mut best_color = BestColor::new(Rgba([0, 0, 0, 255]), 0);

    for (key, value) in color_map.into_iter() {


        let test_color = Srgb::new(key[0], key[1], key[2]);
        let lab_a: Oklab = pixel_color.into_linear().into_color();
        let lab_b: Oklab = test_color.into_linear().into_color();

        let diff = lab_a.hybrid_distance(lab_b);
        if diff < lowest_distance {
            
            lowest_distance = diff;
            best_color = BestColor::new(key.clone().to_rgba(), value.clone());
        }
    }
    
    return best_color;
}

fn get_nearest_color_ciede2000(pixel: &image::Rgba<u8>, color_map: &HashMap<Rgb<u8>, i32>, check_transparency: bool ) -> BestColor {


    // check if the pixel is transparent
    if check_transparency && pixel[3] == 0 {
        return BestColor::new(pixel.clone(), 0);
    }

    let pixel_color = Srgb::new(pixel[0], pixel[1], pixel[2]);

    let mut lowest_distance = f32::INFINITY;

    let mut best_color = BestColor::new(Rgba([0, 0, 0, 255]), 0);

    for (key, value) in color_map.into_iter() {


        let test_color = Srgb::new(key[0], key[1], key[2]);
        let lab_a: Lch = pixel_color.into_linear().into_color();
        let lab_b: Lch = test_color.into_linear().into_color();

        let diff = lab_a.difference(lab_b);
        if diff < lowest_distance {
            
            lowest_distance = diff;
            best_color = BestColor::new(key.clone().to_rgba(), value.clone());
        }
    }
    
    return best_color;
}

fn image_to_makecode_string(img: DynamicImage, color_map: &HashMap<Rgb<u8>, i32>, check_transparency: bool, method: &String) -> (String, image::ImageBuffer<Rgba<u8>, Vec<u8>>) {
    
    let mut preview_img = image::RgbaImage::new(img.width(), img.height());

    let img = img.to_rgba8();

    let mut img_string = "img`".to_string();

    for py in 0..img.height() {
        let mut x_line = "    ".to_string();
        for px in 0..img.width() {
            let mut best_color = BestColor::new(Rgba([0, 0, 0, 255]), 0);
            match method.as_str() {
                "hyab" => {best_color = get_nearest_color_hyab(img.get_pixel(px, py), &color_map, check_transparency);},
                "ciede2000" => {best_color = get_nearest_color_ciede2000(img.get_pixel(px, py), &color_map, check_transparency);}
                "pythagorean" => {best_color = get_nearest_color(img.get_pixel(px, py), &color_map, check_transparency);}
                _ => {best_color = get_nearest_color(img.get_pixel(px, py), &color_map, check_transparency);}
            }
            preview_img.put_pixel(px, py, best_color.color_value);


            x_line += &format!("{:x}",best_color.color_number);
            x_line += " ";
        }
        x_line += &"\n".to_string();
        img_string += &x_line;
    }
    img_string += "`";

    return (img_string, preview_img);
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
pub fn load_image(bytes: &[u8], unparsed_colors: String, colormap_name: String, width: u32, height: u32, check_transparency: bool, conversion_method: String) -> JsValue  {
    
    
    let color_maps= parse_colors(unparsed_colors);
    let color_map = color_maps.get(&colormap_name).expect("Invalid colormap!");

    // guess the image format
    let img_format_guess = guess_format(bytes).expect("Failed to guess image format!");
    
    info!("Guessed image format: {:#?}", img_format_guess);

    if img_format_guess == image::ImageFormat::Gif {
        let cursor = Cursor::new(bytes);
        let decoder = GifDecoder::new(cursor).expect("Failed to initialize gif decoder");
        let frames = decoder.into_frames().collect_frames().expect("Failed to get frames from gif");
        
        let frame_count = frames.len().clone();
        info!("Converting {} frames", frame_count);

        // create a string that will hold the frames
        let mut anim_string = "[".to_string();
        // get the first frame for the preview
        let mut first_frame = DynamicImage::ImageRgba8(frames[0].clone().into_buffer());

        // loop through the frames in the gif
        for (index, frame) in frames.into_iter().enumerate() {
            let mut img = DynamicImage::ImageRgba8(frame.into_buffer());
            img = img.resize(width, height, image::imageops::FilterType::Nearest);
            let converted_img = image_to_makecode_string(img, color_map, check_transparency, &conversion_method);
            let makecode_string = converted_img.0;
            anim_string += &makecode_string;
            anim_string += ",";
            info!("{}%", ((index + 1) as f32 / frame_count as f32) * 100.0);
        }
        anim_string += "]";
        

        first_frame = first_frame.resize(width, height, image::imageops::FilterType::Nearest);
        let converted_img = image_to_makecode_string(first_frame, color_map, check_transparency, &conversion_method);
        let png_img = converted_img.1;
        let mut buffer = Vec::new();
        image::codecs::png::PngEncoder::new(&mut buffer).write_image(&png_img, png_img.width(), png_img.height(), image::ColorType::Rgba8.into()).unwrap();

        let img_data = Uint8Array::from(buffer.as_slice());
        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("pngdata"), &img_data).unwrap();
        Reflect::set(&obj, &JsValue::from_str("makecodedata"), &JsValue::from_str(&anim_string)).unwrap();
        
        // generate code to set the color palette inside makecode
        let mut palette_string = String::new();
        for color in color_map {
            let color_string = format!("color.setColor({}, color.parseColorString(\"{}\"))\n", color.1, rgb_to_hex(color.0));
            palette_string += &color_string;
        }

        Reflect::set(&obj, &JsValue::from_str("colormapgen"), &JsValue::from_str(&palette_string)).unwrap();
        
        return obj.into();
    }

    let mut img = load_from_memory(bytes).expect("failed to load image");

    img = img.resize(width, height, image::imageops::FilterType::Nearest);

    let converted_img = image_to_makecode_string(img, color_map, check_transparency, &conversion_method);
    let makecode_string = converted_img.0;
    let png_img = converted_img.1;

    let mut buffer = Vec::new();
    image::codecs::png::PngEncoder::new(&mut buffer).write_image(&png_img, png_img.width(), png_img.height(), image::ColorType::Rgba8.into()).unwrap();
    
    let img_data = Uint8Array::from(buffer.as_slice());

    let obj = Object::new();
    Reflect::set(&obj, &JsValue::from_str("pngdata"), &img_data).unwrap();
    Reflect::set(&obj, &JsValue::from_str("makecodedata"), &JsValue::from_str(&makecode_string)).unwrap();
    
    // generate code to set the color palette inside makecode
    let mut palette_string = String::new();
    for color in color_map {
        let color_string = format!("color.setColor({}, color.parseColorString(\"{}\"))\n", color.1, rgb_to_hex(color.0));
        palette_string += &color_string;
    }

    Reflect::set(&obj, &JsValue::from_str("colormapgen"), &JsValue::from_str(&palette_string)).unwrap();
    
    return obj.into();
}

#[wasm_bindgen]
pub fn generate_palette_from_img(bytes: &[u8]) -> String {
    let img = load_from_memory(bytes).expect("failed to load image");
    let img = img.to_rgb8();

    // get the colors
    let pixels: Vec<Rgb<u8>> = img.pixels().map(|p| Rgb([p[0], p[1], p[2]])).collect();

    let color_palette = median_cut::generate_palette(pixels);

    // convert to a human readable string
    let mut color_palette_string = "image:\n".to_string();
    for color in color_palette {
        color_palette_string = format!("{}{}\n", color_palette_string, rgb_to_hex(&color));
    }

    return color_palette_string;
}

#[wasm_bindgen]
pub fn get_valid_colormaps(unparsed_colors: String) -> Vec<String> {
    let color_maps= parse_colors(unparsed_colors);
    let mut colormap_names = Vec::new();
    for cm in color_maps {
        colormap_names.push(cm.0);
    }
    return colormap_names;
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen(start)]
pub fn start() {
    let _ = console_log::init_with_level(log::Level::Debug);
    info!("Wasm started!");
}