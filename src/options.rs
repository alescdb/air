use getopts::Options;

pub struct CommandLine {
    pub verbose: bool,
    pub clear: bool,
    pub markdown: bool,
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
        opts.optflag("c", "clear", "Clear history");
        opts.optflag("v", "verbose", "Verbose/debug");
        opts.optflag("m", "markdown", "Display as markdown");
        opts.optflag("h", "help", "Help");

        let matches = match opts.parse(&args[1..]) {
            Ok(matches) => matches,
            Err(fail) => return Err(fail.to_string()),
        };

        let usage = opts.usage(&format!("Usage: {} [options] <prompt>", args[0]));
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
