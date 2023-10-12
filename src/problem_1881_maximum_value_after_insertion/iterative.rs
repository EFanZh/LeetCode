pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let mut n = n;
        let x = char::from(b'0' + (x as u8));

        let index = if n.starts_with('-') {
            n.find(|c| c > x) // We know that '0' > '-'.
        } else {
            n.find(|c| c < x)
        }
        .unwrap_or(n.len());

        n.insert(index, x);

        n
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_value(n: String, x: i32) -> String {
        Self::max_value(n, x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
