use crate::consts::{CONFIG_FILE, GAME_FOLDER};
use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Core {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub config: ProjectConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    project_name: String,
    project_id: String,

    #[serde(default = "Core::default_window_width")]
    window_width: i32,

    #[serde(default = "Core::default_window_height")]
    window_height: i32,

    #[serde(default = "Core::default_grid_size")]
    pub grid_size: i32,
}

impl Core {
    fn default_window_width() -> i32 {
        640
    }

    fn default_window_height() -> i32 {
        480
    }

    fn default_grid_size() -> i32 {
        16
    }

    pub fn get_project_details() -> Result<ProjectConfig, Box<dyn std::error::Error>> {
        let mut cwd = std::env::current_dir()?;
        cwd.push(CONFIG_FILE);

        let data = fs::read_to_string(cwd).expect("Hello World");
        let project_info: ProjectConfig = serde_json::from_str(&data.as_str()).unwrap();

        Ok(project_info)
    }

    pub fn init() -> Core {
        let project_info = Core::get_project_details().unwrap();

        let (rl, thread) = raylib::init()
            .size(*&project_info.window_width, *&project_info.window_height)
            .title(&project_info.project_name)
            .build();

        Core {
            rl,
            thread,
            config: project_info,
        }
    }

    pub fn game_loop<F>(&mut self, cb: F)
    where
        F: 'static + Fn(&mut Core),
    {
        cb(self);
    }

    pub fn get_game_folder() -> PathBuf {
        let mut d = std::env::current_dir().unwrap();
        d.push(GAME_FOLDER);
        d
    }
}
