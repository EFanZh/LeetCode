pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn iter_chunks(values: &[u8], mut f: impl FnMut(usize, usize)) {
        let mut prev = values[0];
        let mut start = 0;
        let mut i = 1;

        while let Some(&value) = values.get(i) {
            if value != prev {
                f(start, i);

                prev = value;
                start = i;
            }

            i += 1;
        }

        f(start, i);
    }

    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::iter_chunks(s.as_bytes(), |start, end| {
            if end - start > 2 {
                result.push(vec![start as _, end as i32 - 1]);
            }
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        Self::large_group_positions(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
