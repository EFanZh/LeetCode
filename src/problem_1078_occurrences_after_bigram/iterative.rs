pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut iter = text.split(' ');
        let mut prev_2 = iter.next().unwrap();
        let mut prev_1 = iter.next().unwrap();

        iter.filter_map(|word| {
            let result = (prev_2 == first && prev_1 == second).then(|| word.to_string());

            prev_2 = prev_1;
            prev_1 = word;

            result
        })
        .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        Self::find_ocurrences(text, first, second)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
