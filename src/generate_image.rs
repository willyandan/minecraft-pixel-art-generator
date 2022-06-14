use crate::entities::errors::image_not_found::ImageNotFound;
use crate::entities::{block::Block, rgb::Rgb};
use image::{self, DynamicImage, GenericImageView};
use std::collections::{HashMap, HashSet};
use std::error::Error;

fn subttract_u8(a: u8, b: u8) -> f32 {
    let x = f32::from(a);
    let y = f32::from(b);
    x - y
}

fn distance_between_pixels(a: &Rgb, b: &Rgb) -> f32 {
    let x = subttract_u8(a.red, b.red);
    let y = subttract_u8(a.green, b.green);
    let z = subttract_u8(a.blue, b.blue);
    f32::sqrt((x * x) + (y * y) + (z * z))
}

fn get_closest_block(pixel: &Rgb, list: &Vec<Block>) -> Block {
    let (block, _) = list
        .into_iter()
        .map(|block| {
            let distance = distance_between_pixels(pixel, &block.color);
            (block, distance)
        })
        .fold(
            (list.get(0).unwrap(), f32::INFINITY),
            |(min_block, min_distance), (block, distance)| match min_distance {
                d if d > distance => (block, distance),
                _ => (min_block, min_distance),
            },
        );
    block.clone()
}

fn create_image(image: DynamicImage, pixels_map: &HashMap<Rgb, Block>) -> String {
    let commands: Vec<String> = image
        .pixels()
        .map(|(x, y, pixel)| {
            let rgb = Rgb::from_tuple((pixel.0[0], pixel.0[1], pixel.0[2]));
            let block = pixels_map.get(&rgb).unwrap();
            format!(
                "setblock ~{} ~{} ~ minecraft:{}",
                x,
                124 - y,
                block.item.name
            )
        })
        .collect();
    commands.join("\n")
}

fn resize_image(image: DynamicImage) -> DynamicImage {
    let nwidth = image.width();
    let nheight = 125;
    image.resize(nwidth, nheight, image::imageops::Lanczos3)
}

fn get_image(image_path: &String) -> Result<DynamicImage, ImageNotFound> {
    match image::open(image_path) {
        Ok(image) => Ok(image),
        Err(_) => Err(ImageNotFound::new(image_path.clone())),
    }
}

pub fn generate_image(image_path: &String) -> Result<String, Box<dyn Error>> {
    let raw_image = get_image(image_path)?;
    let raw_image = resize_image(raw_image);
    let pixel_map: HashSet<Rgb> = raw_image
        .pixels()
        .map(|(_, _, pixel)| Rgb::from_tuple((pixel.0[0], pixel.0[1], pixel.0[2])))
        .collect();

    let block_pixels: Vec<Block> = Block::initialize()?;
    let pixel_map: HashMap<Rgb, Block> = pixel_map
        .into_iter()
        .map(|key| (key.clone(), get_closest_block(&key, &block_pixels)))
        .collect();
    let final_image = create_image(raw_image, &pixel_map);
    Ok(final_image)
}
