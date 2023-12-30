use getopts::Options;

pub struct CommandLine {
    pub verbose: bool,
    pub clear: bool,
    pub markdown: bool,
    pub prompt: String,
    pub usage: String,
}

pub fn parse_command_line(markdown: bool) -> CommandLine {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();

    opts.optflag("c", "clear", "Clear history");
    opts.optflag("v", "verbose", "Verbose/debug");
    opts.optflag("m", "markdown", "Display as markdown");
    opts.optflag("h", "help", "Help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let usage =  opts.usage(&format!("Usage: {} [options] <prompt>", args[0]));
    if matches.opt_present("h") {
        eprintln!("{}", usage);
        std::process::exit(10);
    }
    let mut md: bool = matches.opt_present("m");
    if md == false {
        md = markdown;
    }
    return CommandLine {
        verbose: matches.opt_present("v"),
        clear: matches.opt_present("c"),
        markdown: md,
        prompt: matches.free.join(" ").trim().to_string(),
        usage,
    };
}

pub fn display_options(options: &CommandLine) {
    termimad::print_inline(&format!("*CLEAR*      => `{}`\n", options.clear));
    termimad::print_inline(&format!("*MARKDOWN*   => `{}`\n", options.markdown));
    termimad::print_inline(&format!("*VERBOSE*    => `{}`\n", options.verbose));
    termimad::print_inline(&format!("*PROMPT*     => `{}`\n", options.prompt));
    termimad::print_inline("___\n");
}
