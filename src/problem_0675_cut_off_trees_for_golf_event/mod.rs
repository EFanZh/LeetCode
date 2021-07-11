pub mod bfs;

pub trait Solution {
    fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[&[1, 2, 3] as &[_], &[0, 0, 4], &[7, 6, 5]] as &[&[_]], 6),
            (&[&[1, 2, 3], &[0, 0, 0], &[7, 6, 5]], -1),
            (&[&[2, 3, 4], &[0, 0, 5], &[8, 7, 6]], 6),
            (
                &[
                    &[54_581_641, 64_080_174, 24_346_381, 69_107_959],
                    &[86_374_198, 61_363_882, 68_783_324, 79_706_116],
                    &[668_150, 92_178_815, 89_819_108, 94_701_471],
                    &[83_920_491, 22_724_204, 46_281_641, 47_531_096],
                    &[89_078_499, 18_904_913, 25_462_145, 60_813_308],
                ],
                57,
            ),
        ];

        for (forest, expected) in test_cases {
            assert_eq!(
                S::cut_off_tree(forest.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
