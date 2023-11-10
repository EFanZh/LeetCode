pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let k = k as u32;
        let mut value = 0;

        // Convert and transform.

        for c in s.into_bytes() {
            let c = c - (b'a' - 1);

            value += u16::from(c / 10 + c % 10);
        }

        // Transform.

        for _ in 1..k {
            let mut temp = 0;

            while value != 0 {
                temp += value % 10;
                value /= 10;
            }

            value = temp;
        }

        i32::from(value)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_lucky(s: String, k: i32) -> i32 {
        Self::get_lucky(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
