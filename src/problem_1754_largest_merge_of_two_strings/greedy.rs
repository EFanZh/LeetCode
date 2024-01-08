pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_merge(word1: String, word2: String) -> String {
        let mut iter_1 = word1.as_bytes().iter();
        let mut iter_2 = word2.as_bytes().iter();
        let mut c1 = *iter_1.next().unwrap();
        let mut result = Vec::with_capacity(word1.len() + word2.len());

        'outer: while let Some(&c2) = iter_2.next() {
            loop {
                if (c2, iter_2.as_slice()) > (c1, iter_1.as_slice()) {
                    result.push(c2);

                    break;
                }

                result.push(c1);

                if let Some(&next_c1) = iter_1.next() {
                    c1 = next_c1;
                } else {
                    c1 = c2;
                    iter_1 = iter_2;

                    break 'outer;
                }
            }
        }

        result.push(c1);
        result.extend(iter_1);

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_merge(word1: String, word2: String) -> String {
        Self::largest_merge(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
