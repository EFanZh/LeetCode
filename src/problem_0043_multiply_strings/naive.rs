pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut result = vec![0; num1.len() + num2.len()];

        for (i, lhs) in num1.into_bytes().into_iter().rev().enumerate() {
            for (rhs, target) in num2.bytes().rev().zip(&mut result[i..]) {
                *target += u16::from((lhs - b'0') * (rhs - b'0'));
            }
        }

        if let Some((mut first, mut rest)) = result.split_first_mut() {
            while let Some((second, new_rest)) = rest.split_first_mut() {
                *second += *first / 10;
                *first %= 10;

                first = second;
                rest = new_rest;
            }
        }

        while result.len() > 1 && *result.last().unwrap() == 0 {
            result.pop();
        }

        String::from_utf8(result.into_iter().rev().map(|v| b'0' + v.to_le_bytes()[0]).collect()).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn multiply(num1: String, num2: String) -> String {
        Self::multiply(num1, num2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
