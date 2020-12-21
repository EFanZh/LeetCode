pub struct Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut result = 0;
        let mut lengths = vec![0];

        for line in input.lines() {
            let name = line.trim_start_matches('\t');
            let depth = line.len() - name.len();

            if name.contains('.') {
                result = result.max(lengths[depth] + name.len());
            } else {
                let length = lengths[depth] + name.len() + 1;

                if let Some(slot) = lengths.get_mut(depth + 1) {
                    *slot = length;
                } else {
                    lengths.push(length);
                }
            }
        }

        result as _
    }
}

impl super::Solution for Solution {
    fn length_longest_path(input: String) -> i32 {
        Self::length_longest_path(input)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
