use raylib::prelude::*;

#[derive(Debug)]
pub struct Core {
    pub rl: RaylibHandle,
    pub thread: RaylibThread
}

impl Core {
    pub fn init() -> Core {
        let (rl, thread) = raylib::init()
            .size(640, 480)
            .title("Hello world")
            .build();

        Core {
            rl: rl,
            thread: thread
        }
    }
}
