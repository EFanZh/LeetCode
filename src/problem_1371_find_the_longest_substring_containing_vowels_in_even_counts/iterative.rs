pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut result = 0;
        let mut cache = [0_u32; 32];
        let mut state = 0;

        cache[0] = 1;

        for (i, c) in (2..).zip(s.bytes()) {
            state ^= match c {
                b'a' => 1 << 0,
                b'e' => 1 << 1,
                b'i' => 1 << 2,
                b'o' => 1 << 3,
                b'u' => 1 << 4,
                _ => 0,
            };

            let first_index = &mut cache[state];

            if *first_index == 0 {
                *first_index = i;
            } else {
                result = result.max(i - *first_index);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_longest_substring(s: String) -> i32 {
        Self::find_the_longest_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
