use image::{DynamicImage, Rgb};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, process};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    size: String,

    #[arg(short, long)]
    img: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}


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

    let args = Args::parse();
    let parts: Vec<&str> = args.size.split("x").collect();
    
    if parts.len() != 2 {
        println!("invalid image size specified");
        process::exit(1);
    }



    let width: u32 = parts[0].parse().expect("invalid image size specified");
    let height: u32 = parts[1].parse().expect("invalid image size specified");

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

    if args.img.is_dir() {
        let paths = std::fs::read_dir(&args.img).expect("invalid path specified");

        let mut anim_string = "[".to_string();

        for path in paths {
            let mut img = image::open(path.expect("invalid path specified").path()).unwrap();
            img = img.resize(width, height, image::imageops::FilterType::Nearest);

            let img_string = image_to_makecode_string(img, &color_map);

            anim_string += &img_string;
            anim_string += ",";

        }
        anim_string += "]";

        fs::write(&args.output, anim_string).unwrap();
        
    } else {
        let mut img = image::open(&args.img).unwrap();
        img = img.resize(width, height, image::imageops::FilterType::Nearest);
    
        let img_string = image_to_makecode_string(img, &color_map);
    
        fs::write(&args.output, img_string).unwrap();
    }


}

fn image_to_makecode_string(img: DynamicImage, color_map: &HashMap<Rgb<u8>, i32>) -> String {
    
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

    return img_string;
}