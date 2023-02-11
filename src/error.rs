pub struct ErrorHandler();

impl ErrorHandler {
    pub fn report(&self, line: usize, loc: &str, msg: &str) {
        eprintln!("[line {}] Error{}: {}", line, loc, msg);
    }

    pub fn error(&self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }

    pub fn new() -> ErrorHandler {
        ErrorHandler()
    }
}
