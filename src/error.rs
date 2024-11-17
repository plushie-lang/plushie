use lalrpop_util::ParseError;
use std::fmt;
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub file_name: String,
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
            file_name,
            snippet:
                Snippet {
                    line,
                    column,
                    source,
                },
        } = self;

        write!(
            f,
            "{message}\n> {file_name}:{line}:{column}\n|\n| {source}\n| {pointer}",
            pointer = " ".repeat(column - 1) + "^",
        )
    }
}

impl std::error::Error for Error {}

impl Error {
    fn new(file_name: &str, message: String, source: &str, pos: usize) -> Self {
        let mut line = 1;
        let mut last_newline = 0;

        // Find the line number and last newline position
        for (i, c) in source.chars().take(pos).enumerate() {
            if c == '\n' {
                line += 1;
                last_newline = i + 1;
            }
        }

        let column = source[last_newline..pos].width();

        // Extract the relevant line of code
        let source = source[last_newline..]
            .lines()
            .next()
            .unwrap_or("")
            .to_string();

        Self {
            message,
            file_name: file_name.to_string(),
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
            file_name: "unknown".to_string(),
            snippet: Snippet {
                line: 0,
                column: 0,
                source: String::new(),
            },
        }
    }

    pub fn from_parse_error(
        file_name: &str,
        source: &str,
        error: ParseError<usize, lalrpop_util::lexer::Token, &str>,
    ) -> Error {
        match error {
            ParseError::InvalidToken { location } => Error::new(
                file_name,
                "Invalid token found".to_string(),
                source,
                location,
            ),
            ParseError::UnrecognizedEof { location, expected } => Error::new(
                file_name,
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
                file_name,
                format!(
                    "Unexpected token '{token}'. Expected one of: {}",
                    expected.join(", ")
                ),
                source,
                start,
            ),
            ParseError::ExtraToken {
                token: (start, token, _end),
            } => Error::new(
                file_name,
                format!("Extra token '{token}' found"),
                source,
                start,
            ),
            ParseError::User { .. } => Error::unknown(),
        }
    }
}
