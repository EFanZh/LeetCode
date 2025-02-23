pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut queries = queries;

        queries.retain(|query| {
            dictionary.iter().any(|word| {
                let mut diff_count = 0;

                query.bytes().zip(word.bytes()).all(|(lhs, rhs)| {
                    diff_count += u8::from(lhs != rhs);

                    diff_count < 3
                })
            })
        });

        queries
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        Self::two_edit_words(queries, dictionary)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
