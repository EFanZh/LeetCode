pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let [mut num1, mut num2, mut num3] = [num1, num2, num3].map(i32::cast_unsigned);
        let mut result = (num1 % 10).min(num2 % 10).min(num3 % 10);

        num1 /= 10;
        num2 /= 10;
        num3 /= 10;

        result += 10 * (num1 % 10).min(num2 % 10).min(num3 % 10);

        num1 /= 10;
        num2 /= 10;
        num3 /= 10;

        result += 100 * (num1 % 10).min(num2 % 10).min(num3 % 10);

        num1 /= 10;
        num2 /= 10;
        num3 /= 10;

        (result + 1000 * (num1).min(num2).min(num3)).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        Self::generate_key(num1, num2, num3)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
