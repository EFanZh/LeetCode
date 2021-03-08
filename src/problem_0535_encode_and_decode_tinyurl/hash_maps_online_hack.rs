use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const MIN_CHAR: u8 = b'a';
const MAX_CHAR: u8 = b'z';

struct Inner {
    short_to_long: HashMap<Rc<str>, Rc<str>>,
    long_to_short: HashMap<Rc<str>, Rc<str>>,
    prev: Vec<u8>,
}

impl Inner {
    fn new() -> Self {
        Self {
            short_to_long: HashMap::new(),
            long_to_short: HashMap::new(),
            prev: Vec::new(),
        }
    }

    fn next(s: &mut Vec<u8>) {
        for c in s.iter_mut().rev() {
            if *c == MAX_CHAR {
                *c = MIN_CHAR;
            } else {
                *c += 1;

                return;
            }
        }

        s.insert(0, MIN_CHAR);
    }

    #[allow(clippy::option_if_let_else)]
    fn encode(&mut self, long_url: String) -> String {
        if let Some(short_url) = self.long_to_short.get(long_url.as_str()) {
            short_url.to_string()
        } else {
            Self::next(&mut self.prev);

            let result = String::from_utf8(self.prev.clone()).unwrap();
            let long_url = Rc::<str>::from(long_url);
            let short_url = Rc::<str>::from(result.as_str());

            self.long_to_short.insert(long_url.clone(), short_url.clone());
            self.short_to_long.insert(short_url, long_url);

            result
        }
    }

    fn decode(&self, short_url: String) -> String {
        self.short_to_long[short_url.as_str()].to_string()
    }
}

struct Codec(RefCell<Inner>);

impl Codec {
    fn new() -> Self {
        Self(RefCell::new(Inner::new()))
    }

    fn encode(&self, long_url: String) -> String {
        self.0.borrow_mut().encode(long_url)
    }

    fn decode(&self, short_url: String) -> String {
        self.0.borrow().decode(short_url)
    }
}

impl super::Codec for Codec {
    fn new() -> Self {
        Self::new()
    }

    fn encode(&mut self, long_url: String) -> String {
        (&*self).encode(long_url)
    }

    fn decode(&self, short_url: String) -> String {
        self.decode(short_url)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Codec>();
    }
}
