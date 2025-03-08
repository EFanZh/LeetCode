pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn assign_add(target: &mut u8, value: u8) {
        *target += value;

        if *target >= 26 {
            *target -= 26;
        }
    }

    fn assign_sub(target: &mut u8, value: u8) {
        if *target < value {
            *target += 26;
        }

        *target -= value;
    }

    fn helper(slice: &mut [u8], shifts: &[Vec<i32>]) {
        let mut prev = b'a';

        for offset in &mut *slice {
            let current = *offset;

            Self::assign_sub(offset, prev);

            prev = current;
        }

        for shift in shifts {
            let [start, end, direction] = shift.as_slice().try_into().ok().unwrap();
            let diff = if direction == 0 { 25 } else { 1 };

            Self::assign_add(&mut slice[start as u32 as usize], diff);

            if let Some(end) = slice.get_mut(end as u32 as usize + 1) {
                Self::assign_sub(end, diff);
            }
        }

        let mut prev = b'a';

        for target in slice {
            prev += *target;

            if prev > b'z' {
                prev -= 26;
            }

            *target = prev;
        }
    }

    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut buffer = s.into_bytes();

        Self::helper(&mut buffer, &shifts);

        String::from_utf8(buffer).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        Self::shifting_letters(s, shifts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
