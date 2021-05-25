#[derive(Default)]
struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    has_value: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn add_word(&mut self, word: String) {
        let mut node = self;

        for c in word.into_bytes() {
            node = node.children[(c - b'a') as usize].get_or_insert_with(|| Box::new(Self::new()));
        }

        node.has_value = true;
    }

    fn search_helper(&self, word: &[u8]) -> bool {
        if let Some((&first, rest)) = word.split_first() {
            if let Some(child_slot) = self
                .children
                .get(first.wrapping_sub(b'a') as usize)
                .map(Option::as_deref)
            {
                child_slot.map_or(false, |child| child.search_helper(rest))
            } else {
                for child in self.children.iter().filter_map(Option::as_deref) {
                    if child.search_helper(rest) {
                        return true;
                    }
                }

                false
            }
        } else {
            self.has_value
        }
    }

    fn search(&self, word: String) -> bool {
        self.search_helper(word.as_bytes())
    }
}

impl super::WordDictionary for WordDictionary {
    fn new() -> Self {
        Self::new()
    }

    fn add_word(&mut self, word: String) {
        self.add_word(word);
    }

    fn search(&self, word: String) -> bool {
        self.search(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::WordDictionary>();
    }
}
