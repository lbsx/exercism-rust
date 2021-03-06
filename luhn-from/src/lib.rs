use std::collections::HashSet;

pub struct Luhn {
    s: String,
}

impl Luhn {
    // This function is copied from luhn.
    pub fn is_valid(&self) -> bool {
        let allowed_chars: HashSet<char> =
            ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' ']
                .iter()
                .cloned()
                .collect();

        if !self.s.chars().all(|x| allowed_chars.contains(&x)) {
            return false;
        }

        if self.s.chars().filter(|x| x.is_digit(10)).count() <= 1 {
            return false;
        }

        let sum = self.s
            .chars()
            .filter_map(|x| x.to_digit(10))
            .rev()
            .enumerate()
            .fold(0, |acc, (index, x)| match index % 2 == 0 {
                true => acc + x,
                false if x * 2 > 9 => acc + x * 2 - 9,
                false => acc + x * 2,
            });

        sum % 10 == 0
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(src: T) -> Self {
        Luhn { s: src.to_string() }
    }
}
