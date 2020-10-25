pub struct Solution;

impl Solution {
    fn read_binary_watch_helper(base: u16, start: u8, n: u8, result: &mut Vec<String>) {
        if n == 0 {
            let hour = base >> 6;
            let minute = base & 0b_0011_1111;

            if hour < 12 && minute < 60 {
                result.push(format!("{}:{:02}", hour, minute));
            }
        } else {
            for first in start..11 - n {
                Self::read_binary_watch_helper(base | 1 << first, first + 1, n - 1, result);
            }
        }
    }

    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut result = Vec::new();

        Self::read_binary_watch_helper(0, 0, num as _, &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn read_binary_watch(num: i32) -> Vec<String> {
        Self::read_binary_watch(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
