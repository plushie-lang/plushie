use lalrpop_util::ParseError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub snippet: Snippet,
}

#[derive(Debug)]
pub struct Snippet {
    pub line: usize,
    pub column: usize,
    pub source: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            message,
            snippet:
                Snippet {
                    line,
                    column,
                    source,
                },
            ..
        } = self;

        write!(
            f,
            "{message}\n> <file>.pb:{line}:{column}\n|\n|{source}\n|{pointer}",
            pointer = " ".repeat(column - 1) + "^",
        )
    }
}

impl std::error::Error for Error {}

impl Error {
    fn new(message: String, source: &str, pos: usize) -> Self {
        let mut line = 1;
        let mut last_newline = 0;

        // Find the line number and last newline position
        for (i, c) in source.chars().take(pos).enumerate() {
            if c == '\n' {
                line += 1;
                last_newline = i + 1;
            }
        }

        // Calculate column (account for tabs)
        let column = source[last_newline..pos]
            .chars()
            .fold(0, |acc, c| acc + if c == '\t' { 8 } else { 1 });

        // Extract the relevant line of code
        let source = source[last_newline..]
            .lines()
            .next()
            .unwrap_or("")
            .to_string();

        Self {
            message,
            snippet: Snippet {
                line,
                column,
                source,
            },
        }
    }

    pub fn unknown() -> Self {
        Self {
            message: "Unknown parsing error".to_string(),
            snippet: Snippet {
                line: 0,
                column: 0,
                source: String::new(),
            },
        }
    }

    pub fn from_parse_error(
        source: &str,
        error: ParseError<usize, lalrpop_util::lexer::Token, &str>,
    ) -> Error {
        match error {
            ParseError::InvalidToken { location } => {
                Error::new("Invalid token found".to_string(), source, location)
            }
            ParseError::UnrecognizedEof { location, expected } => Error::new(
                format!(
                    "Unexpected end of file. Expected one of: {}",
                    expected.join(", ")
                ),
                source,
                location,
            ),
            ParseError::UnrecognizedToken {
                token: (start, token, _end),
                expected,
            } => Error::new(
                format!(
                    "Unexpected token '{token}'. Expected one of: {}",
                    expected.join(", ")
                ),
                source,
                start,
            ),
            ParseError::ExtraToken {
                token: (start, token, _end),
            } => Error::new(format!("Extra token '{token}' found"), source, start),
            ParseError::User { .. } => Error::unknown(),
        }
    }
}
