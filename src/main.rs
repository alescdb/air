mod chatgpt;
mod config;
mod options;
mod history;
mod path;
mod logs;
mod llama;
mod ichat;

use crate::logs::*;
use termimad::*;
use crate::chatgpt::ChatGPT;
use crate::config::{display_setup, load_setup, Setup};
use crate::options::{display_options, parse_command_line};
use crate::history::History;
use termimad::crossterm::style::Stylize;
use crate::ichat::{IChat};
use crate::llama::LLamaChat;


fn display(markdown: bool, content: String) {
    if markdown {
        let skin = MadSkin::default();
        println!("{}", skin.term_text(&content.trim()));
    } else {
        println!("{}", content.trim());
    }
}


fn get_chat(setup: &Setup) -> Box<dyn IChat> {
    if setup.llama.enable {
        return Box::new(LLamaChat::new(setup.llama.model.clone()));
    }
    return Box::new(ChatGPT::new(setup.apikey.clone()));
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let setup: Setup = match load_setup() {
        Ok(setup) => setup,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(10);
        }
    };
    let options = parse_command_line(setup.markdown);
    let mut history = History::new(setup.expiration);
    let mut chatgpt = get_chat(&setup);

    unsafe {
        VERBOSE = if cfg!(debug_assertions) { true } else { options.verbose };
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

    chatgpt.set_model(setup.model.to_string());
    if !setup.system.is_empty() {
        chatgpt.set_system(setup.system.to_string());
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

    let answer = chatgpt.chat(options.prompt.to_string(), Some(history.get_completions())).await;
    display(options.markdown, answer.clone());

    history.add(chatgpt.get_name(), &*options.prompt, &*answer.clone());
    history.save();

    Ok(())
}
