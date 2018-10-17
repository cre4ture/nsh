use rustyline::error::ReadlineError;
use std::fs::File;
use std::env;
use std::process;
use rustyline;
use exec;
use parser;
use prompt::{parse_prompt, draw_prompt};
use std::path::{Path, PathBuf};

static DEFAULT_PROMPT: &'static str = "\\W $ ";

fn resolve_and_create_history_file() -> Option<PathBuf> {
    if let Some(home_dir) = env::home_dir() {
        let history_path = Path::new(&home_dir).join(".nsh_history");
        if history_path.exists() {
            return Some(history_path)
        }

        if let Ok(f) = File::create(&history_path) {
            return Some(history_path)
        }
    }

    None
}

pub fn mainloop() {
    let mut rl = rustyline::Editor::<()>::new();

    // Load histories.
    let history_path = resolve_and_create_history_file();
    if let Some(ref path) = history_path {
        rl.load_history(path).expect("failed to load history");
    } else {
        warn!("disabling history");
    }

    loop {
        // Parse and print the prompt.
        let mut prompt_str = String::new();
        let ps1 = env::var("PS1").unwrap_or(DEFAULT_PROMPT.to_owned());
        if let Ok(fmt) = parse_prompt(ps1.as_str()) {
            draw_prompt(&fmt, &mut prompt_str);
        }

        // Read a line.
        let line = match rl.readline(&prompt_str) {
            Ok(line) => line,
            Err(ReadlineError::Eof) => {
                if let Some(ref path) = history_path {
                    rl.save_history(path).expect("failed to save history");
                }
                process::exit(0);
            },
            Err(err) => {
                panic!("something went wrong with readline: {:?}", err);
            }
        };

        rl.add_history_entry(line.as_ref());

        let cmd = match parser::parse_line(line.as_str()) {
            Ok(cmd) => cmd,
            Err(err) => {
                eprintln!("nsh: parse error: {:?}", err);
                return;
            }
        };

        exec::exec(cmd);
    }
}