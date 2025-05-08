pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn helper(money: u8, children: u8) -> u8 {
        money.checked_sub(children).map_or(u8::MAX, |money| {
            let candidate = money / 7;
            let remainder = money % 7;

            let minus_1 = candidate != 0
                && match candidate.cmp(&children) {
                    Ordering::Less => remainder == 3 && candidate == children - 1,
                    Ordering::Equal => remainder != 0,
                    Ordering::Greater => return children - 1,
                };

            candidate - u8::from(minus_1)
        })
    }

    pub fn dist_money(money: i32, children: i32) -> i32 {
        i32::from(Self::helper(money as _, children as _) as i8)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn dist_money(money: i32, children: i32) -> i32 {
        Self::dist_money(money, children)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
