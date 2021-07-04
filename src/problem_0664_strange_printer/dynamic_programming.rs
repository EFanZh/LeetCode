pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Cache {
    buffer: Box<[u8]>,
    n: usize,
}

impl Cache {
    fn new(n: usize) -> Self {
        Self {
            buffer: vec![0; (n - 1) * (n - 1)].into_boxed_slice(),
            n: n - 1,
        }
    }

    fn get(&self, length: usize, start: usize) -> u8 {
        if length < 2 {
            length as _
        } else {
            self.buffer[self.n * (length - 2) + start]
        }
    }

    fn set(&mut self, length: usize, start: usize, value: u8) {
        self.buffer[self.n * (length - 2) + start] = value;
    }
}

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let mut s = s.into_bytes();

        s.dedup();

        let n = s.len();
        let mut cache = Cache::new(n);

        for length in 2..=n {
            for start in 0..=(n - length) {
                let mut value = cache.get(length - 1, start + 1) + 1;
                let (first, rest) = s[start..start + length].split_first().unwrap();

                for (i, c) in rest.iter().enumerate() {
                    if c == first {
                        value = value.min(cache.get(i, start + 1) + cache.get(length - (i + 1), start + 1 + i));
                    }
                }

                cache.set(length, start, value);
            }
        }

        cache.get(n, 0).into()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn strange_printer(s: String) -> i32 {
        Self::strange_printer(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
