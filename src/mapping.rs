use crate::consts::{MAPPING_FILE, SCENE_FOLDER};
use crate::core::Core;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MappingItem {
    pub char: char,
    pub sprite: String,
}

#[derive(Debug)]
pub struct LoadedMappingItem {
    pub char: char,
    pub sprite: String,
    pub texture: Texture2D,
}

type LoadedHashMap = HashMap<String, LoadedMappingItem>;

pub struct Mapping {}

impl Mapping {
    /// read the mapping file into a HashMap
    pub fn read_mapping_file() -> HashMap<String, MappingItem> {
        let mut game = Core::get_game_folder();
        game.push(MAPPING_FILE);
        let contents = fs::read_to_string(game).unwrap();
        let res: HashMap<String, MappingItem> = serde_json::from_str(&*contents).unwrap();

        res
    }

    fn render_sprite(
        d: &mut RaylibDrawHandle,
        texture: &Texture2D,
        row: &usize,
        col: &usize,
        grid_size: &i32,
    ) {
        d.draw_texture(
            texture,
            *col as i32 * grid_size,
            *row as i32 * grid_size,
            Color::WHITE,
        );
    }

    /// render hashmap as sprites with the ability to ignore items
    pub fn render_scene(
        mut d: &mut RaylibDrawHandle,
        scene: &str,
        mapping: &LoadedHashMap,
        grid_size: &i32,
    ) -> std::io::Result<()> {
        let mut scene_path = Core::get_game_folder();
        scene_path.push(SCENE_FOLDER);
        scene_path.push(format!("{}.txt", scene));

        let file = File::open(scene_path)?;
        let reader = BufReader::new(file);
        let mut character_storage: HashMap<char, &Texture2D> = HashMap::new();

        for (row, line) in reader.lines().into_iter().enumerate() {
            let unwrapped = &line.unwrap();
            let chars: Vec<char> = unwrapped.chars().collect();

            for (col, c) in chars.iter().enumerate() {
                if character_storage.contains_key(&c) {
                    Mapping::render_sprite(
                        &mut d,
                        character_storage.get(&c).unwrap(),
                        &row,
                        &col,
                        &grid_size,
                    );
                } else {
                    for (key, value) in mapping {
                        if value.char == *c && key != "empty" {
                            character_storage.insert(*c, &value.texture);
                            Mapping::render_sprite(&mut d, &value.texture, &row, &col, &grid_size);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn preload_scene_textures(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        mapping: &HashMap<String, MappingItem>,
    ) -> LoadedHashMap {
        let mut loaded: LoadedHashMap = HashMap::new();

        for (key, value) in mapping {
            if key != "empty" {
                let mut p = Core::get_game_folder();
                p.push(&value.sprite);

                let text = rl.load_texture(&thread, p.to_str().unwrap()).unwrap();
                let key = key.into();

                loaded.insert(
                    key,
                    LoadedMappingItem {
                        texture: text,
                        char: value.char,
                        sprite: value.sprite.parse().unwrap(),
                    },
                );
            }
        }

        loaded
    }

    /// find instances of a character and returns the row,col
    pub fn find_character_positions(_c: char) -> Vec<(i32, i32)> {
        Vec::new()
    }

    /// translates a row, col into a world position
    pub fn convert_pos_to_world(x: i32, y: i32) -> Vector2 {
        Vector2::new(x as f32, y as f32)
    }
}
