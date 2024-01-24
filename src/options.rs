use getopts::Options;

const PKG_NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
const PKG_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const PKG_AUTHOR: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

pub struct CommandLine {
    pub verbose: bool,
    pub clear: bool,
    pub markdown: bool,
    pub list: bool,
    pub system: Option<String>,
    pub prompt: String,
    pub local: Option<String>,
    pub usage: String,
    pub scan: Option<String>,
}

impl Default for CommandLine {
    fn default() -> CommandLine {
        CommandLine {
            verbose: if cfg!(debug_assertions) { true } else { false },
            clear: false,
            markdown: true,
            list: false,
            system: None,
            prompt: "".to_string(),
            local: None,
            usage: "".to_string(),
            scan: None,
        }
    }
}

#[allow(dead_code)]
impl CommandLine {
    pub fn new(default_system: &str, default_markdown: bool) -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();
        let mut opts = Options::new();

        opts.optopt("l", "local", "Run local model (llama-cpp)", "name");
        opts.optopt("x", "scan", "Scan for local models (llama-cpp)", "folder");
        opts.optflag("L", "list", "List local models (llama-cpp)");
        opts.optflag("c", "clear", "Clear history");
        opts.optflag("v", "verbose", "Verbose/debug");
        opts.optflag("m", "markdown", "Toggle markdown");
        opts.optflag("s", "system-prompt", "Set system prompt (empty for None)");
        opts.optflag("h", "help", "Help");

        let matches = match opts.parse(&args[1..]) {
            Ok(matches) => matches,
            Err(fail) => return Err(fail.to_string()),
        };

        let pname = PKG_NAME.unwrap_or("aid");
        let header = format!(
            "{} v{} (c) {}",
            pname,
            PKG_VERSION.unwrap_or("?.?.?"),
            PKG_AUTHOR.unwrap_or("???")
        );
        let usage = opts.usage(&format!("{}\nUsage: {} [options] <prompt>", header, pname));
        if matches.opt_present("h") {
            return Err(usage);
        }
        // toggle markdown if option -m is present
        let mut md: bool = matches.opt_present("m");
        if md {
            md = !default_markdown;
        } else {
            md = default_markdown;
        }

        let mut sys: String = default_system.to_string();
        if matches.opt_present("m") {
            sys = matches.opt_str("s").unwrap_or("".to_string());
        }

        return Ok(CommandLine {
            verbose: matches.opt_present("v"),
            clear: matches.opt_present("c"),
            markdown: md,
            system: Some(sys),
            prompt: matches.free.join(" ").trim().to_string(),
            local: matches.opt_str("l"),
            list: matches.opt_present("L"),
            scan: matches.opt_str("x"),
            usage,
        });
    }

    pub fn display(&self) {
        termimad::print_inline(&format!("*CLEAR*      => `{}`\n", self.clear));
        termimad::print_inline(&format!("*MARKDOWN*   => `{}`\n", self.markdown));
        termimad::print_inline(&format!("*VERBOSE*    => `{}`\n", self.verbose));
        termimad::print_inline(&format!("*LOCAL*      => `{:?}`\n", self.local));
        termimad::print_inline(&format!("*PROMPT*     => `{}`\n", self.prompt));
        termimad::print_inline(&format!("*SYSTEM*     => `{:?}`\n", self.system));
        termimad::print_inline("___\n");
    }
}
