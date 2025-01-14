pub mod mathematical;

pub trait Solution {
    fn is_strictly_palindromic(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        for n in 4..100_000 {
            assert!(!S::is_strictly_palindromic(n));
        }
    }
}
