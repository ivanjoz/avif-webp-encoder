// build for lambda arm: cargo build --target aarch64-unknown-linux-gnu --release
// export RUSTFLAGS='-C link-arg=-static' && cargo build --release --target x86_64-unknown-linux-musl
use std::io::{self, Read};
fn main() {
    println!("Starting Rust Binary...!!");
    
    /* 
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

        let image_result = ImageReader::open(&image_path);
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
    */
}
/* 
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
    output_cli: bool,
}
*/