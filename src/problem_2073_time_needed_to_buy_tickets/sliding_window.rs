pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as u32 as usize;
        let mut result = 0;
        let target = tickets[k];
        let (left, right) = tickets.split_at(k + 1);

        for &num in left {
            result += num.min(target);
        }

        for &num in right {
            result += num.min(target - 1);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        Self::time_required_to_buy(tickets, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
