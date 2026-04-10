pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut values = values;

        assert_eq!(instructions.len(), values.len());

        let mut i = 0;

        while let Some(instruction) = instructions.get(i) {
            let value = &mut values[i];
            let saved_value = *value;

            if saved_value == i32::MIN {
                break;
            }

            *value = i32::MIN;

            if instruction.bytes().next() == Some(b'a') {
                result += i64::from(saved_value);
                i += 1;
            } else {
                i = i.wrapping_add(saved_value as _);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
        Self::calculate_score(instructions, values)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
