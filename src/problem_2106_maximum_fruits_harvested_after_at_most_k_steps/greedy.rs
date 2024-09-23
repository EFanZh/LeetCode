pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let fruits = fruits
            .into_iter()
            .map(|fruit| {
                let [position, amount]: [_; 2] = fruit.try_into().ok().unwrap();

                (position as u32, amount as u32)
            })
            .collect::<Vec<_>>();

        let start_pos = start_pos as u32;
        let k = k as u32;
        let start_index = fruits.partition_point(|&(position, _)| position < start_pos);
        let mut result = 0;

        // Left.

        let mut i = start_index;
        let mut sum = 0;

        loop {
            i = i.wrapping_sub(1);

            if let Some(&(position, amount)) = fruits.get(i) {
                if position + k < start_pos {
                    break;
                }

                sum += amount;
            } else {
                break;
            }
        }

        let mut j = start_index;

        for &(position, amount) in &fruits[i.wrapping_add(1)..start_index] {
            while let Some(&(right_position, right_amount)) = fruits.get(j) {
                if start_pos - position + (right_position - start_pos) * 2 <= k {
                    sum += right_amount;
                    j += 1;
                } else {
                    break;
                }
            }

            result = u32::max(result, sum);
            sum -= amount;
        }

        // Right.

        while let Some(&(position, amount)) = fruits.get(j) {
            if start_pos + k < position {
                break;
            }

            sum += amount;
            j += 1;
        }

        i = start_index.wrapping_sub(1);

        for &(position, amount) in fruits[start_index..j].iter().rev() {
            while let Some(&(left_position, left_amount)) = fruits.get(i) {
                if position - start_pos + (start_pos - left_position) * 2 <= k {
                    sum += left_amount;
                    i = i.wrapping_sub(1);
                } else {
                    break;
                }
            }

            result = u32::max(result, sum);
            sum -= amount;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        Self::max_total_fruits(fruits, start_pos, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
