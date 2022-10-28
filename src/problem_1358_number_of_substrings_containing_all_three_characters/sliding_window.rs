pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Counter {
    a_count: u32,
    b_count: u32,
    c_count: u32,
}

impl Counter {
    fn get(&mut self, c: u8) -> &mut u32 {
        match c {
            b'a' => &mut self.a_count,
            b'b' => &mut self.b_count,
            _ => &mut self.c_count,
        }
    }
}

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut result = 0;
        let mut start = 0;
        let mut counter = Counter::default();
        let mut zeroes = 3;

        for &c in s {
            let count = counter.get(c);

            if *count == 0 {
                zeroes -= 1;
            }

            *count += 1;

            while zeroes == 0 {
                let count = counter.get(s[start]);

                *count -= 1;

                if *count == 0 {
                    zeroes += 1;
                }

                start += 1;
            }

            result += start;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_substrings(s: String) -> i32 {
        Self::number_of_substrings(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
