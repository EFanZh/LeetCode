pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut indices = [const { Vec::new() }; 26];
        let mut s = s.into_bytes();

        for i in 0..s.len() {
            let c = s[i];

            if let Some(indices) = indices.get_mut(usize::from(c).wrapping_sub(usize::from(b'a'))) {
                indices.push(i as u32);
            } else {
                let index = indices.iter_mut().find_map(Vec::pop).unwrap();

                s[index as usize] = b'*';
            }
        }

        s.retain(|&c| c != b'*');

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn clear_stars(s: String) -> String {
        Self::clear_stars(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
