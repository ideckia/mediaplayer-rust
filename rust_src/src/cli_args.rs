use std::env;

pub struct CliArgs {
    pub sound_path: Option<String>,
    pub loop_sound: bool,
}

impl CliArgs {
    pub fn parse_arguments() -> Self {
        let mut loop_sound = false;
        let mut path_arg_found = false;
        let mut sound_path: Option<String> = None;

        for (index, arg) in env::args().into_iter().enumerate() {
            if index == 0 {
                continue;
            }

            match arg.as_str() {
                "-l" | "--loop" => loop_sound = true,
                "-p" | "--path" => path_arg_found = true,
                _ => {
                    if path_arg_found {
                        path_arg_found = false;
                        sound_path = Some(arg);
                    } else {
                        println!("Ignoring {} argument", arg);
                    }
                }
            }
        }

        Self {
            sound_path,
            loop_sound,
        }
    }
}
