use colored::*;

pub struct LexerError {
    pub line: usize,
    pub column: usize,
    pub msg: String,
    pub hint: Option<String>,
    pub line_content: String,
}

impl LexerError {
    pub fn new(
        line: usize,
        column: usize,
        msg: String,
        hint: Option<String>,
        line_content: String,
    ) -> Self {
        LexerError {
            line,
            column,
            msg,
            hint,
            line_content,
        }
    }

    pub fn report(&self) {
        eprintln!(
            "{}",
            format!(
                "Error at line {} column {}: {}",
                self.line, self.column, self.msg,
            )
            .red()
            .bold()
        );

        // print content from line_content FROM source code
        eprintln!("\n{} | {}", self.line, self.line_content);

        let mut marker = String::new();
        for _ in 0..self.column {
            marker.push(' '); // add space for alignment
        }

        marker.push('^');
        eprintln!("  | {}\n", marker.red());

        // print hint if available
        eprintln!(
            "{}\n",
            self.hint
                .clone()
                .unwrap_or_else(|| "No hints available".to_string())
                .green()
        );
    }
}
