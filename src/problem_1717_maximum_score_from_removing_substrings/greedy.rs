pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let (first, second, x, y) = if y < x { (b'a', b'b', x, y) } else { (b'b', b'a', y, x) };
        let mut num_first = 0;
        let mut num_second = 0;
        let mut result = 0;

        for c in s.into_bytes() {
            if c == first {
                num_first += 1;
            } else if c == second {
                if num_first == 0 {
                    num_second += 1;
                } else {
                    num_first -= 1;
                    result += x;
                }
            } else {
                result += y * num_second.min(num_first);
                num_first = 0;
                num_second = 0;
            }
        }

        result += y * num_second.min(num_first);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        Self::maximum_gain(s, x, y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
