pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut s = s.into_bytes();
        let mut indices = indices;
        let n = s.len();

        for i in 0..n {
            let mut index = indices[i];

            if index != i as i32 {
                let mut c = s[i];

                loop {
                    c = mem::replace(&mut s[index as u32 as usize], c);
                    index = mem::replace(&mut indices[index as u32 as usize], index);

                    if index == i as i32 {
                        break;
                    }
                }

                s[i] = c;
                indices[i] = index;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn restore_string(s: String, indices: Vec<i32>) -> String {
        Self::restore_string(s, indices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
