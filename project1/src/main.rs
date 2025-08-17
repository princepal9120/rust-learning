use clap::Parser;
use maud::{html, Markup, DOCTYPE};
use pulldown_cmark::{html as cmark_html, Options, Parser as MarkdownParser};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(about = "Convert Markdown files to HTML")]
struct Args {
    /// input markdown file
    #[arg(long, short)]
    input: PathBuf,

    /// output html file (optional)
    #[arg(long, short)]
    output: Option<PathBuf>,
}

fn render_html_page(content: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Markdown to HTML" }
            }
            body {
                (maud::PreEscaped(content.to_string()))
            }
        }
    }
}

fn main() {
    let args: Args = Args::parse();
    let markdown_input = fs::read_to_string(&args.input).expect("Failed to read markdown file");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = MarkdownParser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    cmark_html::push_html(&mut html_output, parser);

    let full_html = render_html_page(&html_output).into_string();

    match &args.output {
        Some(path) => {
            fs::write(path, full_html).expect("Failed to write HTML file");
        }
        None => {
            println!("{}", full_html);
        }
    }
}
