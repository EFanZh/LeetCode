pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let (num1, mut num2) = if num2.len() < num1.len() {
            (num2.into_bytes(), num1.into_bytes())
        } else {
            (num1.into_bytes(), num2.into_bytes())
        };

        let mut carry = 0;

        for (lhs, rhs) in num1.iter().rev().zip(num2.iter_mut().rev()) {
            *rhs += lhs + carry - b'0';

            if *rhs < b'0' + 10 {
                carry = 0;
            } else {
                *rhs -= 10;
                carry = 1;
            }
        }

        for rhs in num2.iter_mut().rev().skip(num1.len()) {
            *rhs += carry;

            if *rhs < b'0' + 10 {
                carry = 0;

                break;
            }

            *rhs -= 10;
            carry = 1;
        }

        if carry != 0 {
            num2.insert(0, b'1');
        }

        String::from_utf8(num2).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_strings(num1: String, num2: String) -> String {
        Self::add_strings(num1, num2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
