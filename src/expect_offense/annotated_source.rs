use crate::cop::Offense;
use crate::source::DecodedInput;

pub struct AnnotatedSource {
    lines: Vec<String>,
    annotations: Vec<(usize, String)>,
}

enum ParseStage {
    Whitespace,
    FirstCaret,
    ExtraCarets,
    LeftCurly,
    RightCurly,
}

const ABBREV: &str = "[...]";

impl AnnotatedSource {
    pub fn parse(annotated_code: &str) -> AnnotatedSource {
        let mut lines = Vec::new();
        let mut annotations = Vec::new();

        let mut remaining = annotated_code;
        loop {
            let line_end = remaining.find('\n').unwrap_or(remaining.len());
            let line = &remaining[0..line_end];

            let mut stage = ParseStage::Whitespace;
            let mut is_annotation = false;

            for byte in line.bytes() {
                match stage {
                    ParseStage::Whitespace => {
                        if byte.is_ascii_whitespace() {
                            continue;
                        }
                        if byte == b'^' {
                            stage = ParseStage::FirstCaret;
                            continue;
                        }
                        break;
                    }
                    ParseStage::FirstCaret => {
                        if byte == b'^' {
                            stage = ParseStage::ExtraCarets;
                            continue;
                        }
                        if byte == b'{' {
                            stage = ParseStage::LeftCurly;
                            continue;
                        }
                        if byte == b' ' {
                            is_annotation = true;
                        }
                        break;
                    }
                    ParseStage::ExtraCarets => {
                        if byte == b'^' {
                            continue;
                        }
                        if byte == b' ' {
                            is_annotation = true;
                        }
                        break;
                    }
                    ParseStage::LeftCurly => {
                        if byte == b'}' {
                            stage = ParseStage::RightCurly;
                            continue;
                        }
                        break;
                    }
                    ParseStage::RightCurly => {
                        if byte == b' ' {
                            is_annotation = true;
                        }
                        break;
                    }
                }
            }

            if is_annotation {
                annotations.push((lines.len(), line.to_string()));
            } else {
                lines.push(line.to_string());
            }

            if line_end == remaining.len() {
                break;
            }

            remaining = &remaining[line_end + 1..];
        }

        annotations.sort();

        AnnotatedSource { lines, annotations }
    }
}

impl PartialEq for AnnotatedSource {
    fn eq(&self, other: &AnnotatedSource) -> bool {
        self.lines == other.lines && self.matches_annotations(&other)
    }
}

impl AnnotatedSource {
    fn matches_annotations(&self, other: &AnnotatedSource) -> bool {
        if self.annotations.len() != other.annotations.len() {
            return false;
        }

        for idx in 0..self.annotations.len() {
            let (actual_line, actual_annotation) = &self.annotations[idx];
            let (expected_line, expected_annotation) = &other.annotations[idx];

            if actual_line != expected_line {
                return false;
            }

            if expected_annotation.ends_with(ABBREV) {
                if actual_annotation
                    .starts_with(&expected_annotation[0..expected_annotation.len() - ABBREV.len()])
                {
                    continue;
                }
                return false;
            }

            if actual_annotation != expected_annotation {
                return false;
            }
        }

        true
    }
}

impl std::fmt::Display for AnnotatedSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line_index = 0;
        for &(line_number, ref annotation) in self.annotations.iter() {
            while line_index < line_number {
                writeln!(f, "{}", self.lines[line_index])?;
                line_index += 1;
            }
            writeln!(f, "{}", annotation)?;
        }
        for line in &self.lines[line_index..] {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for AnnotatedSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        std::fmt::Display::fmt(self, f)?;
        Ok(())
    }
}

impl AnnotatedSource {
    pub fn plain_source(&self) -> String {
        self.lines.join("\n")
    }

    pub fn with_offense_annotations(
        &self,
        offenses: &[Offense],
        input: &DecodedInput,
    ) -> AnnotatedSource {
        let mut annotations = Vec::new();

        for offense in offenses {
            let message = &offense.message;

            let (line, column) = input.line_col_for_pos(offense.loc.begin);
            let (end_line, end_column) = input.line_col_for_pos(offense.loc.end);

            let indent = " ".repeat(column);

            let column_length = if line == end_line {
                end_column - column
            } else {
                input.lines[line].len() - column
            };

            let carets = if column_length == 0 {
                "^{}".to_string()
            } else {
                "^".repeat(column_length)
            };

            annotations.push((line + 1, format!("{indent}{carets} {message}")));
        }

        AnnotatedSource {
            lines: self.lines.clone(),
            annotations,
        }
    }
}
