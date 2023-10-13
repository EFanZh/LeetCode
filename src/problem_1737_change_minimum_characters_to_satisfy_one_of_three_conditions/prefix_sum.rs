pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn count_letters(s: String) -> [u32; 26] {
        let mut result = [0; 26];

        for c in s.into_bytes() {
            result[usize::from(c) - usize::from(b'a')] += 1;
        }

        result
    }

    pub fn min_characters(a: String, b: String) -> i32 {
        let length_1 = a.len() as u32;
        let length_2 = b.len() as u32;
        let total_length = length_1 + length_2;
        let counts_1 = Self::count_letters(a);
        let counts_2 = Self::count_letters(b);
        let mut result = u32::MAX;
        let mut sum_1 = 0;
        let mut sum_2 = 0;

        for (&count_1, &count_2) in counts_1.iter().zip(&counts_2).take(25) {
            sum_1 += count_1;
            sum_2 += count_2;

            result = result.min(length_1 - sum_1 + sum_2);
            result = result.min(sum_1 + (length_2 - sum_2));
            result = result.min(total_length - (count_1 + count_2));
        }

        result = result.min(total_length - (counts_1.last().unwrap() + counts_2.last().unwrap()));

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_characters(a: String, b: String) -> i32 {
        Self::min_characters(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
