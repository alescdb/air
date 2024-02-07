mod error;
mod history;
mod ichat;
mod llama;
mod openai;
mod options;
mod path;
mod scan;
mod setup;
mod displayer;

use crate::history::History;
use crate::ichat::IChat;
use crate::llama::LLamaChat;
use crate::options::CommandLine;
use openai::OpenAI;
use scan::scan_folder;
use setup::{LLamaSetup, Setup};
use std::io::Write;
use termimad::{crossterm::style::Stylize, *};

#[allow(dead_code)]
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

fn get_chat(
    local: &Option<String>,
    setup: &Setup,
    options: &CommandLine,
) -> Result<Box<dyn IChat>, String> {
    if let Some(local) = local {
        if let Some(locals) = &setup.local {
            let _name: String = local.clone();
            let llama: Option<&LLamaSetup> = get_local(&locals, &_name);

            if let Some(llama) = llama {
                return Ok(Box::new(LLamaChat::new(llama, options.verbose)));
            } else {
                return Err(format!(
                    "Can't find local model name in setup : '{}'",
                    _name
                ));
            }
        }
    }
    return Ok(Box::new(OpenAI::new(setup.apikey.clone())));
}

fn init_log(verbose: bool) {
    let level = if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Warn
    };

    env_logger::Builder::new()
        .filter_level(level)
        .parse_default_env()
        .format(|buf, record| {
            if record.level() == log::Level::Error {
                writeln!(
                    buf,
                    "{}",
                    Stylize::red(format_args!("{}", record.args()).to_string())
                )
            } else {
                writeln!(buf, "{}", record.args())
            }
        })
        .init();
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // let mut displayer = Displayer::new();
    // displayer.display("input");

    let mut setup: Setup = match Setup::new() {
        Ok(setup) => setup,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(10);
        }
    };
    let options = match CommandLine::new(&setup.get_system(), setup.get_markdown()) {
        Ok(options) => options,
        Err(usage) => {
            println!("{}", usage);
            std::process::exit(10);
        }
    };
    init_log(options.verbose);
    if let Some(scan) = options.scan {
        scan_folder(&mut setup, &scan);
        std::process::exit(0);
    }

    let mut history = History::new(setup.get_expiration());
    let mut ichat = match get_chat(&options.local, &setup, &options) {
        Ok(chat) => chat,
        Err(message) => {
            log::error!("{}", message);
            std::process::exit(5);
        }
    };

    if options.verbose {
        log::info!("Setup Path : {}", path::get_config_directory());
        log::info!("Setup      : {:?}", path::get_config_path("setup.json"));
        log::info!("History    : {:?}", path::get_config_path("history.json"));
        log::info!("___");

        setup.display();
        options.display();
    }

    if options.list {
        println!("Available models:");
        for llm in &setup.local.unwrap_or(Vec::new()) {
            let exists = if llm.model_exist(&llm) {
                Stylize::green("✓").to_string()
            } else {
                Stylize::red("✗").to_string()
            };
            println!("  - {} {}", llm.name, exists);
        }
        std::process::exit(0);
    }

    ichat.set_model(setup.get_model());
    if let Some(system) = options.system {
        if !system.trim().is_empty() {
            ichat.set_system(system);
        }
    }

    match history.load() {
        Ok(_) => {
            if options.clear {
                history.clear();
                match history.save() {
                    Ok(_) => {
                        println!("History cleared.");
                    }
                    Err(e) => {
                        log::error!("{}", e);
                    }
                }
            }
        }
        Err(e) => {
            log::error!("{}", e);
        }
    }

    if options.prompt.is_empty() {
        // don't display usage if --clear
        if !options.clear {
            eprintln!("{}", options.usage);
            log::error!("Empty prompt.");
        }
        std::process::exit(5);
    }

    let answer: String = match ichat
        .chat(options.prompt.to_string(), Some(history.get_completions()))
        .await
    {
        Ok(answer) => answer,
        Err(message) => {
            log::error!("{}", message);
            std::process::exit(5);
        }
    };

    // if ichat.get_name() != "llama" {
    //     display(options.markdown, answer.clone());
    // }

    if answer.trim().len() > 0 {
        history.add(ichat.get_name(), &*options.prompt, &*answer.clone());
        match history.save() {
            Ok(_) => {
                log::info!("History saved.")
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }
    Ok(())
}
