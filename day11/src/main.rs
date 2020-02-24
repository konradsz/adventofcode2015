use std::convert::TryInto;

struct Password([u8; 8]);

impl Password {
    fn new(password: &str) -> Self {
        Password(
            password
                .as_bytes()
                .try_into()
                .expect("8 character password expected"),
        )
    }

    fn is_valid(&self) -> bool {
        self.0
            .windows(3)
            .any(|window| window[1] == window[0] + 1 && window[2] == window[1] + 1)
            && !self
                .0
                .iter()
                .any(|c| *c == 'i' as u8 || *c == 'o' as u8 || *c == 'l' as u8)
            && self.0.windows(2).enumerate().any(|(index, first)| {
                self.0.windows(2).skip(index + 2).any(|second| {
                    first[0] != second[0] && first[0] == first[1] && second[0] == second[1]
                })
            })
    }

    fn get_next_valid(&mut self) -> Option<Password> {
        let mut iter = self.filter(|p| p.is_valid());
        iter.next()
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

impl Iterator for Password {
    type Item = Password;

    fn next(&mut self) -> Option<Password> {
        if self.0.iter().all(|b| *b == 'z' as u8) {
            return None;
        }

        self.0[7] = self.0[7] + 1;

        for i in (0..8).rev() {
            if self.0[i] > 'z' as u8 {
                self.0[i] = 'a' as u8;
                self.0[i - 1] = self.0[i - 1] + 1;
            }
        }
        Some(Password(self.0))
    }
}

fn main() {
    let mut password = Password::new("hepxcrrq");
    if let Some(next_valid) = password.get_next_valid() {
        assert_eq!("hepxxyzz", next_valid.to_string());
    }

    if let Some(next_valid) = password.get_next_valid() {
        assert_eq!("heqaabcc", next_valid.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_validation() {
        assert!(!Password::new("hijklmmn").is_valid());
        assert!(!Password::new("abbceffg").is_valid());
        assert!(!Password::new("abbcegjk").is_valid());
    }

    #[test]
    fn test_password_generation() {
        assert_eq!(
            "abcdffaa",
            Password::new("abcdefgh")
                .get_next_valid()
                .unwrap()
                .to_string()
        );
        assert_eq!(
            "ghjaabcc",
            Password::new("ghijklmn")
                .get_next_valid()
                .unwrap()
                .to_string()
        );
    }
}
