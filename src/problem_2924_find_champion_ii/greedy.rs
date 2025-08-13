pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut lost = vec![false; n as u32 as usize].into_boxed_slice();

        for edge in edges {
            lost[edge[1].cast_unsigned() as usize] = true;
        }

        if lost.iter().filter(|&&lost| !lost).count() == 1 {
            lost.iter().position(|&lost| !lost).unwrap() as _
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        Self::find_champion(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
