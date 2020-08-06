use raylib::prelude::*;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = ".abyss.config.yml";

#[derive(Debug)]
pub struct Core {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    project_name: String,
    project_id: String,

    #[serde(default = "Core::default_window_width")]
    window_width: i32,

    #[serde(default = "Core::default_window_height")]
    window_height: i32,
}

fn get_project_details() -> Result<ProjectConfig, Box<dyn std::error::Error>> {
    let mut cwd = std::env::current_dir()?;
    cwd.push(CONFIG_FILE);

    let data = std::fs::read_to_string(cwd).expect("Hello World");
    let project_info: ProjectConfig = serde_yaml::from_str(&data.as_str()).unwrap();

    Ok(project_info)
}

impl Core {
    fn default_window_width() -> i32 {
        640
    }

    fn default_window_height() -> i32 {
        480
    }

    pub fn init() -> Core {
        let project_info = get_project_details().unwrap();

        let (rl, thread) = raylib::init()
            .size(*&project_info.window_width, *&project_info.window_height)
            .title(&project_info.project_name)
            .build();

        Core { rl, thread }
    }

    pub fn game_loop<State>(&mut self, state: &mut State, f: fn(&mut Core, &mut State)) {
        while !&self.rl.window_should_close() {
            f(self, state);
        }
    }
}
