pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper_1(callback: &mut dyn FnMut(bool) -> bool) {
        // Generates 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 2, 2, ....

        if callback(true) {
            let mut value = false;

            Self::helper_1(&mut |count| {
                if callback(value) && (!count || callback(value)) {
                    value = !value;

                    true
                } else {
                    false
                }
            });
        }
    }

    fn helper_0(callback: &mut dyn FnMut(bool) -> bool) {
        if callback(false) && callback(true) {
            Self::helper_1(callback);
        }
    }

    pub fn magical_string(mut n: i32) -> i32 {
        let mut result = 0;

        Self::helper_0(&mut |value| {
            result += i32::from(!value);
            n -= 1;
            n != 0
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn magical_string(n: i32) -> i32 {
        Self::magical_string(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
