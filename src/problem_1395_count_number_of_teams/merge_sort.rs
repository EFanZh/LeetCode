pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn merge_to_buffer(data: &[(u32, u16, u16)], buffer: &mut [(u32, u16, u16)]) {
        let (left_data, right_data) = data.split_at(data.len() / 2);
        let mut left_iter = left_data.iter().copied();
        let mut right_iter = right_data.iter().copied();
        let mut left = left_iter.next().unwrap();
        let mut right = right_iter.next().unwrap();
        let mut right_used = 0;
        let mut writer = buffer.iter_mut();
        let mut write = |value| *writer.next().unwrap() = value;

        loop {
            if right.0 < left.0 {
                right_used += 1;
                write(right);

                if let Some(new_right) = right_iter.next() {
                    right = new_right;
                } else {
                    left.2 += right_used;
                    write(left);

                    for (target, mut value) in writer.zip(left_iter) {
                        value.2 += right_used;
                        *target = value;
                    }

                    break;
                }
            } else {
                left.2 += right_used;
                write(left);

                if let Some(new_left) = left_iter.next() {
                    left = new_left;
                } else {
                    write(right);

                    for (target, value) in writer.zip(right_iter) {
                        *target = value;
                    }

                    break;
                }
            }
        }
    }

    fn merge_sort_to_buffer(data: &mut [(u32, u16, u16)], buffer: &mut [(u32, u16, u16)]) {
        let n = data.len();

        if n > 1 {
            let (left, right) = data.split_at_mut(n / 2);

            Self::merge_sort_to_self(left, buffer);
            Self::merge_sort_to_self(right, buffer);
            Self::merge_to_buffer(data, buffer);
        } else {
            buffer[..n].copy_from_slice(data);
        }
    }

    fn merge_sort_to_self(data: &mut [(u32, u16, u16)], buffer: &mut [(u32, u16, u16)]) {
        let n = data.len();

        if n > 1 {
            let (left, right) = data.split_at_mut(n / 2);
            let (buffer_left, buffer_right) = buffer.split_at_mut(n / 2);

            Self::merge_sort_to_buffer(left, buffer_left);
            Self::merge_sort_to_buffer(right, buffer_right);
            Self::merge_to_buffer(&buffer[..n], data);
        }
    }

    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let n = rating.len();

        let mut data = rating
            .into_iter()
            .zip(0..)
            .map(|(value, i)| (value as _, i, 0))
            .collect::<Vec<_>>();

        Self::merge_sort_to_self(&mut data, &mut vec![(0, 0, 0); n]);

        let n = n as u16;
        let mut result = 0;

        for (new_index, (_, old_index, right_lesser)) in (0..).zip(data) {
            let left_lesser = new_index - right_lesser;
            let left_greater = old_index - left_lesser;
            let right_greater = (n - 1 - new_index) - left_greater;

            result +=
                u32::from(left_lesser) * u32::from(right_greater) + u32::from(left_greater) * u32::from(right_lesser);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_teams(rating: Vec<i32>) -> i32 {
        Self::num_teams(rating)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
