pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let mut order = order;
        let mut is_friend = vec![false; order.len()].into_boxed_slice();

        for friend in friends {
            is_friend[friend.cast_unsigned() as usize - 1] = true;
        }

        order.retain(|id| is_friend[id.cast_unsigned() as usize - 1]);

        order
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        Self::recover_order(order, friends)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
