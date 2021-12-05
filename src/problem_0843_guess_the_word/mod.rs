use std::cell::Cell;

pub mod brute_force;
pub mod brute_force_2;
pub mod brute_force_3;

pub struct Master {
    secret: String,
    num_guesses: Cell<u8>,
    success: Cell<bool>,
}

impl Master {
    pub fn new(secret: String, num_guesses: u8) -> Self {
        Self {
            secret,
            num_guesses: Cell::new(num_guesses),
            success: Cell::new(false),
        }
    }

    pub fn guess(&self, word: String) -> i32 {
        self.num_guesses.set(self.num_guesses.get().checked_sub(1).unwrap());

        let mut result = 0;

        for (lhs, rhs) in self.secret.bytes().zip(word.bytes()) {
            if lhs == rhs {
                result += 1;
            }
        }

        if result == self.secret.len() {
            self.success.set(true);
        }

        result as _
    }

    pub fn success(&self) -> bool {
        self.success.get()
    }
}

pub trait Solution {
    fn find_secret_word(words: Vec<String>, master: &Master);
}

#[cfg(test)]
mod tests {
    use super::{Master, Solution};

    pub fn run<S: Solution>() {
        let test_cases = [
            ("acckzz", &["acckzz", "ccbazz", "eiowzz", "abcczz"] as &[_], 10),
            ("hamada", &["hamada", "khaled"], 10),
            (
                "hbaczn",
                &[
                    "gaxckt", "trlccr", "jxwhkz", "ycbfps", "peayuf", "yiejjw", "ldzccp", "nqsjoa", "qrjasy", "pcldos",
                    "acrtag", "buyeia", "ubmtpj", "drtclz", "zqderp", "snywek", "caoztp", "ibpghw", "evtkhl", "bhpfla",
                    "ymqhxk", "qkvipb", "tvmued", "rvbass", "axeasm", "qolsjg", "roswcb", "vdjgxx", "bugbyv", "zipjpc",
                    "tamszl", "osdifo", "dvxlxm", "iwmyfb", "wmnwhe", "hslnop", "nkrfwn", "puvgve", "rqsqpq", "jwoswl",
                    "tittgf", "evqsqe", "aishiv", "pmwovj", "sorbte", "hbaczn", "coifed", "hrctvp", "vkytbw", "dizcxz",
                    "arabol", "uywurk", "ppywdo", "resfls", "tmoliy", "etriev", "oanvlx", "wcsnzy", "loufkw", "onnwcy",
                    "novblw", "mtxgwe", "rgrdbt", "ckolob", "kxnflb", "phonmg", "egcdab", "cykndr", "lkzobv", "ifwmwp",
                    "jqmbib", "mypnvf", "lnrgnj", "clijwa", "kiioqr", "syzebr", "rqsmhg", "sczjmz", "hsdjfp", "mjcgvm",
                    "ajotcx", "olgnfv", "mjyjxj", "wzgbmg", "lpcnbj", "yjjlwn", "blrogv", "bdplzs", "oxblph", "twejel",
                    "rupapy", "euwrrz", "apiqzu", "ydcroj", "ldvzgq", "zailgu", "xgqpsr", "wxdyho", "alrplq", "brklfk",
                ],
                10,
            ),
        ];

        for (secret, words, num_guesses) in test_cases {
            let master = Master::new(secret.to_string(), num_guesses);

            S::find_secret_word(words.iter().copied().map(str::to_string).collect(), &master);

            assert!(master.success());
        }
    }
}
