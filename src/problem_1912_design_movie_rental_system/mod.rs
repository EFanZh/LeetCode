pub mod btree_set;
pub mod btree_set_2;

pub trait MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self;
    fn search(&self, movie: i32) -> Vec<i32>;
    fn rent(&mut self, shop: i32, movie: i32);
    fn drop(&mut self, shop: i32, movie: i32);
    fn report(&self) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::MovieRentingSystem;

    enum Operation {
        Search(i32, &'static [i32]),
        Rent(i32, i32),
        Drop(i32, i32),
        Report(&'static [[i32; 2]]),
    }

    pub fn run<M: MovieRentingSystem>() {
        let test_cases = [
            (
                (
                    3,
                    &[[0, 1, 5], [0, 2, 6], [0, 3, 7], [1, 1, 4], [1, 2, 7], [2, 1, 5]] as &[_],
                ),
                &[
                    Operation::Search(1, &[1, 0, 2]),
                    Operation::Rent(0, 1),
                    Operation::Rent(1, 2),
                    Operation::Report(&[[0, 1], [1, 2]]),
                    Operation::Drop(1, 2),
                    Operation::Search(2, &[0, 1]),
                ] as &[_],
            ),
            (
                (
                    10,
                    &[
                        [0, 418, 3],
                        [9, 5144, 46],
                        [2, 8986, 29],
                        [6, 1446, 28],
                        [3, 8215, 97],
                        [7, 9105, 34],
                        [6, 9105, 30],
                        [5, 1722, 94],
                        [9, 528, 40],
                        [3, 850, 77],
                        [3, 7069, 40],
                        [8, 1997, 42],
                        [0, 8215, 28],
                        [7, 4050, 80],
                        [4, 7100, 97],
                        [4, 9686, 32],
                        [4, 2566, 93],
                        [2, 8320, 12],
                        [2, 5495, 56],
                    ],
                ),
                &[
                    Operation::Search(7837, &[]),
                    Operation::Search(5495, &[2]),
                    Operation::Rent(4, 7100),
                    Operation::Search(9105, &[6, 7]),
                    Operation::Search(1446, &[6]),
                    Operation::Report(&[[4, 7100]]),
                    Operation::Search(9869, &[]),
                    Operation::Drop(4, 7100),
                ],
            ),
        ];

        for ((n, entries), operations) in test_cases {
            let mut movie_renting_system = M::new(n, entries.iter().map(Vec::from).collect());

            for operation in operations {
                match *operation {
                    Operation::Search(movie, expected) => assert_eq!(movie_renting_system.search(movie), expected),
                    Operation::Rent(shop, movie) => movie_renting_system.rent(shop, movie),
                    Operation::Drop(shop, movie) => movie_renting_system.drop(shop, movie),
                    Operation::Report(expected) => assert_eq!(movie_renting_system.report(), expected),
                }
            }
        }
    }
}
