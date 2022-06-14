use std::{env, error::Error, fs, path::Path};

use crate::entities::errors::{
    invalid_minecraft_configuration::InvalidMinecraftConfiguration,
    write_permission::WritePermission,
};

fn is_inside_minecraft_folder() -> bool {
    let args: Vec<String> = env::args().collect();
    let current_folder = fs::canonicalize(Path::new(&args[0])).unwrap();
    let current_folder = current_folder.parent();
    match current_folder {
        None => false,
        Some(current_folder) => {
            let folder_name = String::from(current_folder.file_name().unwrap().to_str().unwrap());
            folder_name == ".minecraft"
        }
    }
}

fn world_exists(world_name: &String) -> bool {
    let args: Vec<String> = env::args().collect();
    let current_folder = fs::canonicalize(Path::new(&args[0])).unwrap();
    let current_folder = current_folder.parent().unwrap();
    let current_folder = current_folder.to_str().unwrap();
    let path = format!("{}\\saves\\{}", current_folder, world_name);
    let path = Path::new(&path);
    path.exists()
}

fn create_function(
    world: &String,
    function: &String,
    name: &String,
) -> Result<(), WritePermission> {
    let path = format!(
        "./saves/{}/datapacks/pixelart/data/pixelart/functions/{}.mcfunction",
        world, name
    );
    let path = Path::new(&path);
    let content = function.as_bytes();
    if let Err(_) = fs::write(path, content) {
        return Err(WritePermission {});
    }
    Ok(())
}

fn create_datapack_files(world: &String) -> Result<(), WritePermission> {
    let data_pack_path = format!(
        "./saves/{}/datapacks/pixelart/data/pixelart/functions",
        world
    );
    if let Err(_) = fs::create_dir_all(data_pack_path) {
        return Err(WritePermission {});
    }
    let pack_mc =
        String::from("{\"pack\":{\"pack_format\":10,\"description\":\"Pixel art generator\"}}");
    let pack_mc = pack_mc.as_bytes();
    let pack_mc_path = format!("./saves/{}/datapacks/pixelart/pack.mcmeta", world);
    let pack_mc_path = Path::new(&pack_mc_path);
    if let Err(_) = fs::write(pack_mc_path, pack_mc) {
        return Err(WritePermission {});
    }
    Ok(())
}

fn insert_function_into_datapack(
    function: &String,
    world: &String,
    name: &String,
) -> Result<(), WritePermission> {
    let path = format!(
        "./saves/{}/datapacks/pixelart/data/pixelart/functions",
        world
    );
    let path = Path::new(&path);
    if !path.exists() {
        create_datapack_files(world)?;
    }
    create_function(world, function, name)
}

pub fn save_function_into_world(
    function: &String,
    world: &String,
    name: &String,
) -> Result<(), Box<dyn Error>> {
    if is_inside_minecraft_folder() && world_exists(world) {
        insert_function_into_datapack(function, world, name)?;
    } else {
        return Err(Box::new(InvalidMinecraftConfiguration));
    }
    Ok(())
}
