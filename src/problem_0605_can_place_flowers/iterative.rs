pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n == 0 {
            true
        } else {
            let (&first, rest) = flowerbed.split_first().unwrap();
            let mut prev = (0, first);

            for &num in rest {
                if (prev, num) == ((0, 0), 0) {
                    if n == 1 {
                        return true;
                    }

                    n -= 1;

                    prev = (1, 0);
                } else {
                    prev = (prev.1, num)
                }
            }

            prev == (0, 0) && n == 1
        }
    }
}

impl super::Solution for Solution {
    fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        Self::can_place_flowers(flowerbed, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
