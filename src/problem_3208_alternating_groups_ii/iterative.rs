pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let k = k.cast_unsigned();
        let right = &colors[colors.len() - (k as usize - 1)..];
        let mut prev = 0;
        let mut length = 0;
        let mut result = 0;

        right.iter().chain(&colors).for_each(|&color| {
            if color == prev {
                length = 1;
            } else {
                length += 1;
                prev = color;
            }

            result += i32::from(length >= k);
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        Self::number_of_alternating_groups(colors, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
