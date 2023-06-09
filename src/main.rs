use lib_ruby_parser::{Parser, ParserOptions};

use rubocop::cop::Offense;
use rubocop::source::DecodedInput;
use rubocop::{Commissioner, Config, Reporter};

fn main() {
    let mut files = Vec::new();

    let mut args = std::env::args().skip(1).peekable();
    match args.peek() {
        Some(_) => {
            for s in args {
                investigate_recursive(&s, &[&s], &mut files);
            }
        }
        None => {
            investigate_recursive(std::env::current_dir().unwrap(), &[], &mut files);
        }
    };

    let no_color = match std::env::var("NO_COLOR") {
        Ok(s) => s != "",
        Err(_) => false,
    };

    Reporter::new(no_color).print_report(files);
}

fn investigate_recursive<P>(
    path: P,
    components: &[&str],
    files: &mut Vec<(DecodedInput, Vec<Offense>)>,
) where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    if path.is_file() {
        let Some(extension) = path.extension() else { return };
        if extension == "rb" {
            let name = components.join(std::path::MAIN_SEPARATOR_STR);
            files.push(investigate_file(path, name));
        }
    } else {
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();

            let name = entry.file_name();
            let name = name.to_string_lossy();

            // TODO: config
            static EXCLUDE: &[&str] = &["node_modules", "tmp", "vendor", ".git"];
            if EXCLUDE.contains(&name.as_ref()) {
                continue;
            }

            let components = &[components, &[&name]].concat();
            investigate_recursive(entry.path(), components, files);
        }
    }
}

fn investigate_file<P>(path: P, buffer_name: String) -> (DecodedInput, Vec<Offense>)
where
    P: AsRef<std::path::Path>,
{
    let parser_options = ParserOptions {
        buffer_name,
        record_tokens: false,
        ..Default::default()
    };

    let (input, offenses, corrector) = Commissioner::investigate(
        rubocop::default::cops(),
        Config::new(None),
        Parser::new(std::fs::read(path).unwrap(), parser_options),
    );

    let output = corrector.process(input.as_shared_bytes());
    let output = String::from_utf8_lossy(&output);
    let _ = output;
    // println!("{output}");

    (input, offenses)
}
