#[allow(dead_code)]
mod color;
use color::*;

use lib_ruby_parser::Loc;

use crate::cop::Offense;
use crate::source::DecodedInput;

pub struct Reporter {
    no_color: bool,
}

impl Reporter {
    pub fn new(no_color: bool) -> Reporter {
        Reporter { no_color }
    }

    fn set_color(&self, color: Color) {
        if !self.no_color {
            print!("\x1b[{}m", color.fg_code());
        }
    }

    fn reset_color(&self) {
        if !self.no_color {
            print!("\x1b[0m");
        }
    }

    pub fn print_report(&self, mut files: Vec<(DecodedInput, Vec<Offense>)>) {
        let file_count = files.len();
        let offense_count = files.iter().map(|(_, v)| v.len()).sum::<usize>();
        let correctable_count = {
            files
                .iter()
                .map(|(_, v)| v.iter().filter(|o| o.correctable).count())
                .sum::<usize>()
        };

        files.sort_by(|(a, _), (b, _)| a.name.cmp(&b.name));

        for (input, mut offenses) in files {
            offenses.sort_by_key(|Offense { loc, .. }| (loc.begin, loc.end));

            for offense in offenses {
                self.print_offense(&input, offense);
            }
        }

        println!();

        print!("{} file(s) inspected, ", file_count);

        self.set_color(Color::Red);
        print!("{} offense(s)", offense_count);
        self.reset_color();
        print!(" detected");

        if correctable_count > 0 {
            print!(", ");
            self.set_color(Color::Yellow);
            print!("{} offense(s)", correctable_count);
            self.reset_color();
            print!(" autocorrectable");
        }

        println!();
    }

    fn print_offense(&self, input: &DecodedInput, offense: Offense) {
        let Offense {
            loc,
            cop_name,
            message,
            correctable,
        } = offense;

        let (begin_line, begin_col) = input.line_col_for_pos(loc.begin);
        let (end_line, end_col) = input.line_col_for_pos(loc.end);

        assert_eq!(begin_line, end_line, "TODO");

        self.set_color(Color::Cyan);
        print!("{}", input.name);
        self.reset_color();
        print!(":{}:{}: ", begin_line + 1, begin_col + 1);

        if correctable {
            self.set_color(Color::Yellow);
            print!("[Correctable]");
            self.reset_color();
            print!(" ");
        }

        print!("{cop_name}: ");

        if self.no_color {
            println!("{message}");
        } else {
            let mut remaining: &str = &message;
            while let Some(index) = remaining.find('`') {
                print!("{}", &remaining[0..index]);
                remaining = &remaining[index + 1..];
                let index = remaining.find('`').unwrap();
                self.set_color(Color::Yellow);
                print!("{}", &remaining[0..index]);
                self.reset_color();
                remaining = &remaining[index + 1..];
            }
            println!("{remaining}");
        }

        {
            let line = &input.lines[begin_line];
            let source = input.source(Loc {
                begin: line.start,
                end: line.end,
            });
            print!("{source}");
            if line.ends_with_eof {
                println!();
            }
        }

        for _ in 0..begin_col {
            print!(" ");
        }
        for _ in begin_col..end_col {
            print!("^");
        }
        println!();
    }
}
