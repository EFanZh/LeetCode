// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct BrowserHistory {
    homepage: Box<str>,
    history: Vec<String>,
    cursor: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            homepage: homepage.into_boxed_str(),
            history: Vec::new(),
            cursor: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.history.splice(self.cursor.., [url]);
        self.cursor += 1;
    }

    fn go_to(&mut self, cursor: usize) -> String {
        let result = self
            .history
            .get(cursor.wrapping_sub(1))
            .map_or_else(|| self.homepage.as_ref(), String::as_str)
            .to_string();

        self.cursor = cursor;

        result
    }

    fn back(&mut self, steps: i32) -> String {
        self.go_to(self.cursor.saturating_sub(steps as _))
    }

    fn forward(&mut self, steps: i32) -> String {
        self.go_to((self.cursor + steps as usize).min(self.history.len()))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::BrowserHistory for BrowserHistory {
    fn new(homepage: String) -> Self {
        Self::new(homepage)
    }

    fn visit(&mut self, url: String) {
        self.visit(url);
    }

    fn back(&mut self, steps: i32) -> String {
        self.back(steps)
    }

    fn forward(&mut self, steps: i32) -> String {
        self.forward(steps)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::BrowserHistory>();
    }
}
