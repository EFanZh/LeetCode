pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let rows = rows as u32 as usize;
        let columns = encoded_text.len() / rows;
        let mut result = Vec::with_capacity(encoded_text.len());
        let mut iter = encoded_text.bytes();

        for _ in 0..columns {
            result.extend(iter.clone().step_by(columns + 1));
            iter.next();
        }

        result.truncate(result.iter().rposition(|&c| c != b' ').map_or(0, |i| i + 1));

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        Self::decode_ciphertext(encoded_text, rows)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
