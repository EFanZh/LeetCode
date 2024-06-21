pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str::Bytes;

struct Context<'a> {
    iter: Bytes<'a>,
    result: i32,
}

impl Context<'_> {
    #[inline(never)]
    fn start(&mut self) {
        match self.iter.next() {
            None => {}
            Some(b'.') => self.empty(),
            Some(_) => self.hamster(),
        }
    }

    fn empty(&mut self) {
        match self.iter.next() {
            None => {}
            Some(b'.') => self.empty(),
            Some(_) => self.empty_hamster(),
        }
    }

    fn hamster(&mut self) {
        match self.iter.next() {
            Some(b'.') => {
                self.result += 1;

                self.food();
            }
            _ => self.result = -1,
        }
    }

    fn empty_hamster(&mut self) {
        self.result += 1;

        match self.iter.next() {
            None => {}
            Some(b'.') => self.food(),
            Some(_) => self.hamster(),
        }
    }

    fn food(&mut self) {
        match self.iter.next() {
            None => {}
            Some(b'.') => self.empty(),
            Some(_) => self.start(),
        }
    }
}

impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let mut context = Context {
            iter: hamsters.bytes(),
            result: 0,
        };

        context.start();

        context.result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_buckets(hamsters: String) -> i32 {
        Self::minimum_buckets(hamsters)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
