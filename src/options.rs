use getopts::Options;

const PKG_NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
const PKG_VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const PKG_AUTHOR: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

pub struct CommandLine {
    pub verbose: bool,
    pub clear: bool,
    pub markdown: bool,
    pub list: bool,
    pub prompt: String,
    pub local: Option<String>,
    pub usage: String,
}

impl Default for CommandLine {
    fn default() -> CommandLine {
        CommandLine {
            verbose: if cfg!(debug_assertions) { true } else { false },
            clear: false,
            markdown: true,
            list: false,
            prompt: "".to_string(),
            local: None,
            usage: "".to_string(),
        }
    }
}

#[allow(dead_code)]
impl CommandLine {
    pub fn new(default_markdown: bool) -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();
        let mut opts = Options::new();

        opts.optopt("l", "local", "Run local model (llama-cpp)", "name");
        opts.optflag("L", "list", "List local models (llama-cpp)");
        opts.optflag("c", "clear", "Clear history");
        opts.optflag("v", "verbose", "Verbose/debug");
        opts.optflag("m", "markdown", "Display as markdown");
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
        let mut md: bool = matches.opt_present("m");
        if md == false {
            md = default_markdown;
        }
        return Ok(CommandLine {
            verbose: matches.opt_present("v"),
            clear: matches.opt_present("c"),
            markdown: md,
            prompt: matches.free.join(" ").trim().to_string(),
            local: matches.opt_str("l"),
            list: matches.opt_present("list"),
            usage,
        });
    }

    pub fn display(&self) {
        termimad::print_inline(&format!("*CLEAR*      => `{}`\n", self.clear));
        termimad::print_inline(&format!("*MARKDOWN*   => `{}`\n", self.markdown));
        termimad::print_inline(&format!("*VERBOSE*    => `{}`\n", self.verbose));
        termimad::print_inline(&format!("*LOCAL*      => `{:?}`\n", self.local));
        termimad::print_inline(&format!("*PROMPT*     => `{}`\n", self.prompt));
        termimad::print_inline("___\n");
    }
}
