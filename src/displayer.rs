use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::io::Write;
use std::thread;
use std::time::Duration;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Clone)]
pub struct Displayer {
    #[allow(dead_code)]
    markdown: bool,
    buffer: String,
    codeblock: bool,
}

#[allow(dead_code)]
impl Displayer {
    pub fn new(markdown: bool) -> Displayer {
        Displayer {
            markdown: markdown,
            buffer: String::new(),
            codeblock: false,
        }
    }

    fn print_line(&mut self, line: &str) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        if self.codeblock {
            if line == "```" {
                self.codeblock = false;
                stdout.reset().unwrap();
                write!(&mut stdout, "\n").unwrap();
            } else {
                write!(&mut stdout, "{}\n", line).unwrap();
            }
            return;
        }

        let parser: Parser<'_> = Parser::new(&line);
        for event in parser {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Strong => {
                        stdout
                            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
                            .unwrap();
                    }
                    Tag::Emphasis => {
                        stdout
                            .set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_italic(true))
                            .unwrap();
                    }
                    Tag::BlockQuote => {
                        stdout
                            .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_italic(true))
                            .unwrap();
                    }
                    _ => {
                        // println!(" (DEFAULT) {:?}", tag);
                    }
                },
                Event::End(tag) => match tag {
                    TagEnd::Strong => {
                        stdout.reset().unwrap();
                    }
                    TagEnd::Emphasis => {
                        if !self.codeblock {
                            stdout.reset().unwrap();
                        }
                    }
                    TagEnd::CodeBlock => {
                        self.codeblock = !self.codeblock;

                        if self.codeblock {
                            stdout
                                .set_color(
                                    ColorSpec::new().set_bg(Some(Color::Rgb(0x2a, 0x2a, 0x2a))),
                                )
                                .unwrap();
                            writeln!(&mut stdout, "").unwrap();
                        } else {
                            stdout.reset().unwrap();
                        }
                    }
                    _ => {
                        stdout.reset().unwrap();
                    }
                },
                Event::Text(text) => {
                    write!(&mut stdout, "{}", text).unwrap();
                }
                Event::Code(text) => {
                    stdout
                        .set_color(
                            ColorSpec::new()
                                .set_bg(Some(Color::Rgb(0x2a, 0x2a, 0x2a)))
                                .set_fg(Some(Color::Black)),
                        )
                        .unwrap();
                    write!(&mut stdout, " {} ", text).unwrap();
                    stdout.reset().unwrap();
                }
                _ => {
                    println!(" (UNKNOWN) {:?}", event);
                }
            }
        }
        println!("");
    }

    pub fn display_markdown(&mut self, text: &str) {
        for c in text.chars() {
            if c == '\n' {
                // print!("\x1B[2K\r");
                // print!("\x1B[1F");
                // print!("\x1B[0K");
                // print!("\r");
                print!("\x1B[2K\r");
                // print!("\x1B[2K\x1B[1F\x1B[1G"); // clear whole line

                self.print_line(&self.buffer.clone());
                self.buffer = String::new();
            } else {
                self.buffer += &c.to_string();
                print!("{}", c);
                std::io::stdout().flush().unwrap();
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    #[allow(dead_code)]
    pub fn display(&mut self, text: &str) {
        if !self.markdown {
            print!("{}", text);
            std::io::stdout().flush().unwrap();
        } else {
            self.display_markdown(text);
        }
    }
}
