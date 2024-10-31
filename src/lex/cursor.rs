use super::LexError;

pub(crate) struct Cursor<'a> {
    line: usize,
    col: usize,

    /// Position of current cursor, or the current token's end position.
    current_index: usize,
    /// Position of current token's start position, behind [`current_index`].
    current_token_start: usize,

    source: &'a str,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(source: &'a str) -> Self {
        Self {
            line: 1,
            col: 1,
            current_index: 0,
            current_token_start: 0,
            source,
        }
    }

    pub(crate) fn is_end(&self) -> bool {
        self.peek_token().is_none()
    }

    /// Returns a range for access to `source` that gives the raw token string.
    fn get_next_token_range(&self) -> std::ops::Range<usize> {
        fn is_separator(c: char) -> bool {
            c.is_whitespace() || c == '\n'
        }

        let next_token_start = self.source[self.current_index..]
            .find(|c| !is_separator(c))
            .map_or(self.source.len(), |i| i + self.current_index);
        let next_token_end = self.source[next_token_start..]
            .trim_start()
            .find(is_separator)
            .map_or(self.source.len(), |i| i + next_token_start);

        next_token_start..next_token_end
    }

    pub(crate) fn peek_token(&self) -> Option<&'a str> {
        let token_range = self.get_next_token_range();
        if token_range.is_empty() {
            return None;
        }

        Some(&self.source[token_range])
    }

    /// Gets token and advances the cursor.
    pub(crate) fn next_token(&mut self) -> Option<&'a str> {
        let token_range = self.get_next_token_range();
        if token_range.is_empty() {
            return None;
        }

        let advanced_lines = self.source[self.current_index..token_range.end]
            .chars()
            .filter(|&c| c == '\n')
            .count();
        self.line += advanced_lines;

        if advanced_lines != 0 {
            self.col = 1;
        }
        self.col += self.source[self.current_index..token_range.end]
            .lines()
            .last()
            .unwrap()
            .chars()
            .count();

        self.current_index = token_range.end;
        self.current_token_start = token_range.start;
        Some(&self.source[token_range])
    }

    /// Gets the remaining characters in the current line.
    pub(crate) fn current_remaining_line(&mut self) -> &'a str {
        let remaining_end = self.source[self.current_index..]
            .find('\n')
            .unwrap_or(self.source.len());
        let ret = if self
            .source
            .get(self.current_index + remaining_end - 1..=self.current_index + remaining_end)
            == Some("\r\n")
        {
            &self.source[self.current_index..self.current_index + remaining_end - 1]
        } else {
            &self.source[self.current_index..self.current_index + remaining_end]
        };
        self.col += ret.chars().count();
        self.current_token_start = self.current_index;
        self.current_index += remaining_end;
        ret.trim()
    }

    pub(crate) fn line(&self) -> usize {
        self.line
    }

    pub(crate) fn col(&self) -> usize {
        self.col
    }

    pub(crate) fn err_expected_token(&self, message: &'static str) -> LexError {
        log::error!(
            "Expected token {} at line {}, col {} - but found {}",
            message,
            self.line(),
            self.col(),
            &self.source[self.current_token_start..self.current_index]
        );

        LexError::ExpectedToken {
            line: self.line(),
            col: self.col(),
            message,
        }
    }
}
