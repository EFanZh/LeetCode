pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {
        let mut instructions = indices
            .iter()
            .zip(&sources)
            .zip(&targets)
            .map(|((&index, source), target)| (index as usize, source.as_str(), target.as_str()))
            .collect::<Vec<_>>();

        instructions.sort_unstable_by_key(|&(i, _, _)| i);

        let mut result = String::with_capacity(s.len());
        let mut i = 0;

        for &(index, source, target) in &instructions {
            if s[index..].starts_with(source) {
                result.push_str(&s[i..index]);
                result.push_str(target);
                i = index + source.len();
            }
        }

        result.push_str(&s[i..]);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {
        Self::find_replace_string(s, indices, sources, targets)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
