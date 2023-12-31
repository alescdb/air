mod chatgpt;
mod config;
mod history;
mod ichat;
mod llama;
mod logs;
mod options;
mod path;

use crate::chatgpt::ChatGPT;
use crate::config::{display_setup, load_setup, Setup};
use crate::history::History;
use crate::ichat::IChat;
use crate::llama::LLamaChat;
use crate::logs::*;
use crate::options::{display_options, parse_command_line};
use config::LLamaSetup;
use termimad::crossterm::style::Stylize;
use termimad::*;

fn display(markdown: bool, content: String) {
    if markdown {
        let skin = MadSkin::default();
        println!("{}", skin.term_text(&content.trim()));
    } else {
        println!("{}", content.trim());
    }
}

fn get_local<'a>(local: &'a [LLamaSetup], name: &str) -> Option<&'a LLamaSetup> {
    for l in local {
        if l.name.eq(name) {
            return Some(l);
        }
    }
    return None;
}


fn get_chat(local: &Option<String>, setup: &Setup) -> Result<Box<dyn IChat>, String> {
    if let Some(local) = local {
        let _name: String = local.clone();
        let llama: Option<&LLamaSetup> = get_local(&setup.local, &_name);

        if let Some(llama) = llama {
            return Ok(Box::new(LLamaChat::new(
                llama.model.clone(),
                llama.prompt.clone(),
                setup.main_gpu.clone(),
            )));
        } else {
            return Err(format!(
                "Can't find local model name in setup : '{}'",
                _name
            ));
        }
    }
    return Ok(Box::new(ChatGPT::new(setup.apikey.clone())));
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let setup: Setup = match load_setup() {
        Ok(setup) => setup,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(10);
        }
    };
    let options = parse_command_line(setup.markdown);
    let mut history = History::new(setup.expiration);
    let mut ichat = match get_chat(&options.local, &setup) {
        Ok(n) => n,
        Err(e) => {
            error!("{}", e);
            std::process::exit(5);
        }
    };

    unsafe {
        VERBOSE = if cfg!(debug_assertions) {
            true
        } else {
            options.verbose
        };
    }
    verbose!("Configuration Path : {}", path::get_config_directory());
    verbose!("History File : {:?}", path::get_config_path("history.json"));
    verbose!("Setup File : {:?}", path::get_config_path("setup.json"));

    unsafe {
        if VERBOSE {
            display_setup(&setup);
            display_options(&options);
        }
    }

    ichat.set_model(setup.model.to_string());
    if !setup.system.is_empty() {
        ichat.set_system(setup.system.to_string());
    }

    history.load();
    if options.clear {
        history.clear();
        history.save();
        println!("History cleared.");
    }

    if options.prompt.is_empty() {
        // don't display usage if --clear
        if !options.clear {
            eprintln!("{}", options.usage);
            error!("Empty prompt.");
        }
        std::process::exit(5);
    }

    let answer = ichat
        .chat(options.prompt.to_string(), Some(history.get_completions()))
        .await;
    display(options.markdown, answer.clone());

    history.add(ichat.get_name(), &*options.prompt, &*answer.clone());
    history.save();

    Ok(())
}
