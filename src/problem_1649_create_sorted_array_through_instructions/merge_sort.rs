pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Writer<'a> {
    buffer: &'a mut [(u32, u32, u32)],
    cursor: usize,
    left_written: u32,
    last_left: u32,
    last_left_count: u32,
}

impl<'a> Writer<'a> {
    fn new(buffer: &'a mut [(u32, u32, u32)]) -> Self {
        Self {
            buffer,
            cursor: 0,
            left_written: 0,
            last_left: 0,
            last_left_count: 0,
        }
    }

    fn write(&mut self, value: (u32, u32, u32)) {
        self.buffer[self.cursor] = value;
        self.cursor += 1;
    }

    fn write_left(&mut self, value: (u32, u32, u32)) {
        if value.0 == self.last_left {
            self.last_left_count += 1;
        } else {
            self.last_left = value.0;
            self.last_left_count = 1;
        }

        self.left_written += 1;

        self.write(value);
    }

    fn write_right(&mut self, mut value: (u32, u32, u32)) {
        value.1 += self.left_written;

        if value.0 == self.last_left {
            value.1 -= self.last_left_count;
        }

        self.write(value);
    }
}

impl Solution {
    fn merge_to_buffer(left: &[(u32, u32, u32)], right: &[(u32, u32, u32)], buffer: &mut [(u32, u32, u32)]) {
        let mut left_iter = left.iter();
        let mut left = *left_iter.next().unwrap();
        let mut right_iter = right.iter();
        let mut right = *right_iter.next().unwrap();
        let mut writer = Writer::new(buffer);

        loop {
            if right.0 < left.0 {
                writer.write_right((right.0, right.1, right.2 + (left_iter.len() + 1) as u32));

                if let Some(next_right) = right_iter.next() {
                    right = *next_right;
                } else {
                    // Right iterator is exhausted, copy left remaining items to buffer.

                    writer.write(left);

                    writer.buffer[writer.cursor..].copy_from_slice(left_iter.as_slice());

                    break;
                }
            } else {
                writer.write_left(left);

                if let Some(next_left) = left_iter.next() {
                    left = *next_left;
                } else {
                    // Left iterator is exhausted, process right items one by one.

                    writer.write_right(right);

                    for (target, item) in writer.buffer[writer.cursor..].iter_mut().zip(right_iter) {
                        *target = *item;

                        target.1 += writer.left_written;

                        if item.0 == writer.last_left {
                            target.1 -= writer.last_left_count;
                        }
                    }

                    break;
                }
            }
        }
    }

    fn merge_sort_to_buffer(data: &mut [(u32, u32, u32)], buffer: &mut [(u32, u32, u32)]) {
        let n = data.len();

        if n > 1 {
            let middle = n / 2;
            let (data_left, data_right) = data.split_at_mut(middle);
            let (buffer_left, buffer_right) = buffer.split_at_mut(middle);

            Self::merge_sort_to_self(data_left, buffer_left);
            Self::merge_sort_to_self(data_right, buffer_right);
            Self::merge_to_buffer(data_left, data_right, buffer);
        } else {
            buffer.copy_from_slice(data);
        }
    }

    fn merge_sort_to_self(data: &mut [(u32, u32, u32)], buffer: &mut [(u32, u32, u32)]) {
        let n = data.len();

        if n > 1 {
            let middle = n / 2;
            let (data_left, data_right) = data.split_at_mut(middle);
            let (buffer_left, buffer_right) = buffer.split_at_mut(middle);

            Self::merge_sort_to_buffer(data_left, buffer_left);
            Self::merge_sort_to_buffer(data_right, buffer_right);
            Self::merge_to_buffer(buffer_left, buffer_right, data);
        }
    }

    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let n = instructions.len();
        let mut buffer = vec![(0, 0, 0); n * 2].into_boxed_slice();
        let (data, buffer) = buffer.split_at_mut(n);

        for (target, source) in data.iter_mut().zip(instructions) {
            target.0 = source as _;
        }

        Self::merge_sort_to_self(data, buffer);

        let mut result = 0_u64;

        for &mut (_, less_count, greater_count) in data {
            result += u64::from(less_count.min(greater_count));
        }

        (result % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        Self::create_sorted_array(instructions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
