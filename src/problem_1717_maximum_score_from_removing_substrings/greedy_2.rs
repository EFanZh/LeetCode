pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check<const A: u8, const B: u8>(s: &str, x: i32, y: i32) -> i32 {
        let mut num_first = 0;
        let mut num_second = 0;
        let mut result = 0;

        for c in s.bytes() {
            if c == A {
                num_first += 1;
            } else if c == B {
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

    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        if y < x {
            Self::check::<b'a', b'b'>(&s, x, y)
        } else {
            Self::check::<b'b', b'a'>(&s, y, x)
        }
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
