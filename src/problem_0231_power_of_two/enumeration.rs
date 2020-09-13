pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        match n {
            0x0000_0001 | 0x0000_0002 | 0x0000_0004 | 0x0000_0008 | 0x0000_0010 | 0x0000_0020 | 0x0000_0040
            | 0x0000_0080 | 0x0000_0100 | 0x0000_0200 | 0x0000_0400 | 0x0000_0800 | 0x0000_1000 | 0x0000_2000
            | 0x0000_4000 | 0x0000_8000 | 0x0001_0000 | 0x0002_0000 | 0x0004_0000 | 0x0008_0000 | 0x0010_0000
            | 0x0020_0000 | 0x0040_0000 | 0x0080_0000 | 0x0100_0000 | 0x0200_0000 | 0x0400_0000 | 0x0800_0000
            | 0x1000_0000 | 0x2000_0000 | 0x4000_0000 => true,
            _ => false,
        }
    }
}

impl super::Solution for Solution {
    fn is_power_of_two(n: i32) -> bool {
        Self::is_power_of_two(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
