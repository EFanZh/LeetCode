pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut cache = vec![0; n * n];

        for row in 1..n {
            for column in 0..n - row {
                let end = column + row;
                let mut money = column + cache[n * (row - 1) + (column + 1)];

                for i in column + 1..end {
                    money =
                        money.min(i + cache[n * (i - (column + 1)) + column].max(cache[n * (end - 1 - i) + (i + 1)]));
                }

                money = money.min(end + cache[n * (row - 1) + column]);

                cache[n * row + column] = money + 1;
            }
        }

        cache[n * (n - 1)] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_money_amount(n: i32) -> i32 {
        Self::get_money_amount(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
