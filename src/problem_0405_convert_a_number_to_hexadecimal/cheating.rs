pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn to_hex(num: i32) -> String {
        let num = u32::from_le_bytes(num.to_le_bytes());

        format!("{:x}", num)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn to_hex(num: i32) -> String {
        Self::to_hex(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
