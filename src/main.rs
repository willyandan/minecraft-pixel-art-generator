mod entities;
mod generate_image;
mod minecraft_datapack;
use std::{collections::HashMap, env};

struct Args {
    filename: String,
    world_name: String,
    name: String,
}

fn handle_args(args: &Vec<String>) -> Args {
    let hash_arguments: HashMap<String, String> = args
        .into_iter()
        .filter(|argument| argument.starts_with("--"))
        .map(|argument| {
            let b: Vec<&str> = argument.split("=").collect();
            let key = b[0].replace("--", "");
            let value = String::from(b[1]);
            (key, value)
        })
        .collect();

    Args {
        filename: hash_arguments.get("filename").unwrap().trim().to_string(),
        world_name: hash_arguments.get("world").unwrap().trim().to_string(),
        name: hash_arguments.get("name").unwrap().trim().to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = handle_args(&args);
    let content = generate_image::generate_image(&args.filename);
    if let Err(e) = content {
        let error_message = format!("Failed to create image: {}", e.to_string());
        println!("{}", error_message);
        return;
    }

    match minecraft_datapack::save_function_into_world(
        &content.unwrap(),
        &args.world_name,
        &args.name,
    ) {
        Ok(_) => {
            println!("Image successfully translated to minecraft function");
            println!("To use it run those commands above inside your world: ");
            println!("/reload");
            println!("/function pixelart:{}", args.name);
        }
        Err(e) => {
            let error_message = format!("Failed to create image: {}", e.to_string());
            println!("{}", error_message);
        }
    }
}
