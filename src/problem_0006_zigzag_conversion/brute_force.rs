pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;
        let cycle_length = num_rows * 2 - 2;
        let chunks = s.as_bytes().chunks(cycle_length);
        let mut result = vec![0; s.len()];
        let mut iter = result.iter_mut();

        for &c in chunks.clone().filter_map(<[_]>::first) {
            *iter.next().unwrap() = c;
        }

        for row in 1..num_rows - 1 {
            for chunk in chunks.clone() {
                if let Some(&c) = chunk.get(row) {
                    *iter.next().unwrap() = c;

                    if let Some(&c) = chunk.get(cycle_length - row) {
                        *iter.next().unwrap() = c;
                    }
                }
            }
        }

        for &c in chunks.filter_map(|chunk| chunk.get(num_rows - 1)) {
            *iter.next().unwrap() = c;
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn convert(s: String, num_rows: i32) -> String {
        Self::convert(s, num_rows)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
