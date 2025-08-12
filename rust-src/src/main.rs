use image::{imageops, DynamicImage, GenericImageView, Rgba, RgbaImage};
use load_image::{self, export::rgb::{self}};
use base64::prelude::*;
use base64::engine::general_purpose::{STANDARD};
use base64::Engine;
use webp;
use serde_json;
use serde::Serialize;
use std::io::{self, Read};
use thumbhash;
// build for lambda arm: cargo build --target aarch64-unknown-linux-musl --release
// build for lambda amd64: cargo build --target x86_64-unknown-linux-musl --release
fn main() {
    let mut convert_args = ConverArgs {
        image: DynamicImage::new_rgba8(0, 0),
        resolutions: vec![],
        output_directory: "".to_string(),
        name: "".to_string(),
        webp_quality: 82,
        webp_method: 6,
        avif_quality: 82,
        avif_speed: 3,
        use_webp: false,
        use_avif: false,
        use_thumbhash: 0,
        output_cli: false,
    };

    //get execution arguments in a variable
    let args: Vec<String> = std::env::args().collect();
    let current_dir = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
    convert_args.output_directory = current_dir.clone();

    let mut first_arg: String = "".to_owned();
    for (_, _arg) in args.iter().enumerate(){
        let arg = _arg.trim();

        if arg.len() > 7 && &arg[0..7] == "-image=" || arg == "-image-stdin" {
            first_arg = arg.to_string();
            break;
        }
        if &arg[0..1] == "-" {
            break;
        } else {
            first_arg = arg.to_string();
        }
    }

    let mut image_data = Vec::new();

    // Image as base64 on args
    if first_arg.len() > 7 && &first_arg[0..7] == "-image=" {
        let base64_image = &first_arg[7..];
        image_data = BASE64_STANDARD.decode(base64_image).unwrap();
    // Image on stdin
    } else if first_arg == "-image-stdin" {
        io::stdin().read_to_end(&mut image_data).expect("Failed to read image from stdin");
        //print the binary image_data as string, just the first 100 byte
        let mut string_image_data = String::new();
        let mut count = 0;
        for byte in image_data.clone() {
            string_image_data.push_str(&format!("{:02x}", byte));
            count += 1;
            if count >= 100 {
                break;
            }
        }
        print!("Image Data |{}",string_image_data)
    }
    
    if image_data.len() > 0 {
        // read the image data as a DynamicImage object
        let image = image::load_from_memory(&image_data);
        if image.is_err(){
            println!("Error: reading image: {}",image.err().unwrap().to_string());
            return;
        }
        convert_args.image = image.unwrap();
        println!("Reading image from stdin is ok!");
        if convert_args.name.len() == 0 {
            convert_args.name = "image".to_string();
        }
        convert_args.output_cli = true;
    } else {
        let image_path: String;
        if &first_arg[0..1] == "-" {
            println!("Error: must provide an image (name, path or base64) in the first argument");
            return;
        }
        if &first_arg[0..1] == "/" {
            image_path = first_arg;
        } else if &first_arg[0..2] == "./" {
            image_path = current_dir.clone() + &first_arg[1..];
        } else {
            image_path = current_dir.clone() + "/" + &first_arg;
        }
        let image_paths: Vec<&str> = image_path.split("/").collect();
        convert_args.name = image_paths[image_paths.len()-1].to_string();
        
        println!("Reading image: {}",image_path);

        let image_result = image::ImageReader::open(&image_path);
        if image_result.is_err(){
            println!("Error: reading image: {}",image_result.err().unwrap().to_string());
            return;
        }
        let image = image_result.unwrap().decode();
        if image.is_err(){
            println!("Error: reading image: {}",image.err().unwrap().to_string());
            return;
        }
        convert_args.image = image.unwrap();
    }

    for (_, arg) in args.iter().enumerate(){
        if arg == "-webp" {
            convert_args.use_webp = true
        } else if arg == "-avif" {
            convert_args.use_avif = true
        } else if arg.contains("=") {
            let args_equals: Vec<&str> = arg.split("=").collect();
            let arg_name = args_equals[0];
            let arg_value = args_equals[1];
            if arg_name == "-webp-quality" {
                convert_args.use_webp = true;
                convert_args.webp_quality = arg_value.parse::<u32>().unwrap();
            } else if arg_name == "-webp-method" {
                convert_args.use_avif = true;
                convert_args.webp_method = arg_value.parse::<i32>().unwrap();
            } else if arg_name == "-avif-quality" {
                convert_args.use_avif = true;
                convert_args.avif_quality = arg_value.parse::<u32>().unwrap();
            } else if arg_name == "-avif-speed" {
                convert_args.use_avif = true;
                convert_args.avif_speed = arg_value.parse::<u8>().unwrap();
            } else if arg_name == "-output" {
                convert_args.output_directory = arg_value.to_string();
            } else if arg_name == "-resolutions" {
                let resolutions: Vec<&str> = arg_value.split(",").collect();
                for resolution in resolutions {
                    let vu32 = resolution.parse::<u32>();
                    if vu32.is_err() {
                        println!("Error: Invalid resolution format: {}", vu32.unwrap_err().to_string());
                        return;
                    }
                    let resolution = vu32.unwrap();
                    if resolution > 4000 {
                        println!("Error: Invalid resolution: {} | Too Big (>16 mpx)", resolution);
                        return;
                    }
                    convert_args.resolutions.push(resolution);
                }
            }
        }
    }

    println!("Convertig: WEBP={} | AVIF={} | OUTPUT_CLI={}", convert_args.use_webp, convert_args.use_avif, convert_args.output_cli);
    convert_image(convert_args);
}

fn convert_image(args: ConverArgs) {
    
    let mut dimensions: Vec<(u32, u32, u32)> = Vec::new();
    let original_size = args.image.height() * args.image.width();

    for resolution in args.resolutions {
        let new_size = resolution * resolution;
        let proportion_square = (new_size as f32) / (original_size as f32);
        let proportion = proportion_square.sqrt();

        let new_width = (args.image.width() as f32 * proportion) as u32;
        let new_height = (args.image.height() as f32 * proportion) as u32;

        dimensions.push((resolution, new_width, new_height));   
    }

    let mut thumbnail: Option<RgbaImage> = None;
    let mut thumbhash_base64_string: String = "".to_string();

    if args.use_thumbhash == 2 {
        // --- Code for white-padded, centered thumbnail ---
        let target_size = 100;
        let (width, height) = args.image.dimensions();
        
        // Calculate the new size to fit proportionally within 100x100
        let (new_width, new_height) = {
            let ratio = width as f64 / height as f64;
            if ratio > 1.0 { // Landscape or square
                (target_size, (target_size as f64 / ratio).round() as u32)
            } else { // Portrait
                ((target_size as f64 * ratio).round() as u32, target_size)
            }
        };

        // Resize the original image to the new proportional dimensions
        let resized_img = imageops::resize(
            &args.image,
            new_width,
            new_height,
            imageops::FilterType::Triangle,
        );

        // Create a new 100x100 white canvas
        let mut canvas = RgbaImage::from_pixel(target_size, target_size, Rgba([255, 255, 255, 255]));

        // Calculate the position to paste the resized image (centered)
        let x_offset = (target_size - new_width) / 2;
        let y_offset = (target_size - new_height) / 2;

        // Paste the resized image onto the canvas
        imageops::overlay(&mut canvas, &resized_img, x_offset.into(), y_offset.into());

        thumbnail = Some(canvas);
        
    } else if args.use_thumbhash == 1 {
        // --- Code for the default, non-padded resize ---
        thumbnail = Some(imageops::resize(
            &args.image,
            100,
            100,
            imageops::FilterType::Triangle,
        ));
    }

    if args.use_thumbhash > 0 {
        if let Some(thumbnail) = &thumbnail {
            // Get the image dimensions and raw RGBA pixel data from the final 'thumbnail'
            let (width, height) = thumbnail.dimensions();
            let rgba_data = thumbnail.as_raw();

            // Generate the ThumbHash from the pixel data.
            let thumbhash_vec = thumbhash::rgba_to_thumb_hash(
                width as usize,
                height as usize,
                &rgba_data,
            );

            thumbhash_base64_string = STANDARD.encode(&thumbhash_vec);
        } else {
            println!("Error: Thumbnail was not generated for thumbhash.");
            return;
        }
    }

    // Crea los archivos .webp
    for (resolution, width, height) in dimensions {
        let image_resized = args.image.resize(
            width, height, image::imageops::FilterType::Triangle);
        if args.use_webp { // WEBP
            println!("Conviertiendo imagen .webp en dimension: {}x{}...", width, height);
            let encoder: webp::Encoder = webp::Encoder::from_image(&image_resized).unwrap();
    
            let mut config = webp::WebPConfig::new().unwrap();
            config.lossless = 0;
            config.alpha_compression = 1;
            config.quality = args.webp_quality as f32;
            config.method = args.webp_method; // quality/speed trade-off (0=fast, 6=slower-better)
    
            // Encode the image at a specified quality 0-100
            let webp:  webp::WebPMemory = encoder.encode_advanced(&config).unwrap();
            let file_name = format!("{}-{}i-{}x{}.{}", args.name, resolution, width, height, "webp");

            if args.output_cli {

                let output = OutputCmd{
                    image: BASE64_STANDARD.encode(&*webp),
                    name: file_name,
                    resolution: resolution,
                    format: "webp".to_string(),
                    thumbhash: thumbhash_base64_string.clone()
                };

                let output_json = serde_json::to_string(&output).unwrap();
                println!("{}",output_json);

            } else {
                let file_name_webp = format!("{}/{}",args.output_directory, file_name);
                std::fs::write(&file_name_webp, &*webp).unwrap();
        
                println!("Imagen WEBP guardada en: {}", &file_name_webp);
            }
        }
        if args.use_avif { // AVIF
            let mut image_vec1s: Vec<rgb::RGBA<u8>> = vec![];

            for i in 0..image_resized.height() { 
                for j in 0..image_resized.width() {
                    let pixel = image_resized.get_pixel(j, i);
                    let r = pixel[0];
                    let g = pixel[1];
                    let b = pixel[2];
                    let a = pixel[3];
                    image_vec1s.push(rgb::RGBA { r, g, b, a });
                }
            }
    
            println!("Conviertiendo imagen .avif en dimension: {}x{}...", width, height);
        
            // create a new variable of type imgref::ImgVec<RGBA8> and instanciate it with the above image
            let image_vec1 = imgref::ImgVec::new(image_vec1s, image_resized.width() as usize, image_resized.height() as usize);
            let image_img: imgref::Img<&[rgb::RGBA<u8>]> = imgref::Img::new(image_vec1.buf(), image_resized.dimensions().0 as usize, image_resized.dimensions().1 as usize);
        
            let res = ravif::Encoder::new()
                .with_quality(args.avif_quality as f32)
                .with_speed(args.avif_speed)
                .encode_rgba( image_img);
            
            if res.is_err(){
                println!("Error encoding image: {}", res.err().unwrap().to_string());
                return;
            }
            
            let avif_file = res.unwrap().avif_file;
            let file_name = format!("{}-{}i-{}x{}.{}", args.name, resolution, width, height, "avif");

            if args.output_cli {

                let output = OutputCmd{
                    image: BASE64_STANDARD.encode(avif_file),
                    name: file_name,
                    resolution: resolution,
                    format: "avif".to_string(),
                    thumbhash: thumbhash_base64_string.clone()
                };

                let output_json = serde_json::to_string(&output).unwrap();
                println!("{}",output_json);

            } else {
                let file_name_avif = format!("{}/{}",args.output_directory, file_name);
                    
                let result_saved = std::fs::write(&file_name_avif, avif_file);
                if result_saved.is_err() {
                    println!("Error saving image: {}", result_saved.err().unwrap().to_string());
                    return; 
                }
        
                println!("Imagen AVIF guardada en: {}", &file_name_avif);
            }
        }
    }
}


struct ConverArgs {
    image: DynamicImage,
    resolutions: Vec<u32>,
    output_directory: String,
    name: String,
    webp_quality: u32,
    webp_method: i32,
    avif_quality: u32,
    avif_speed: u8,
    use_webp: bool,
    use_avif: bool,
    use_thumbhash: u32,
    output_cli: bool,
}

#[derive(Serialize)]
struct OutputCmd {
    image: String,
    name: String,
    resolution: u32,
    format: String,
    thumbhash: String,
}