pub trait StrExt {
    fn split_tokens(&self) -> Iter<'_>;
}

#[derive(Debug)]
pub struct Iter<'a> {
    s: &'a str,
}

impl<'s> StrExt for &'s str {
    fn split_tokens(&self) -> Iter<'s> {
        Iter { s: self }
    }
}

impl StrExt for String {
    fn split_tokens(&self) -> Iter<'_> {
        Iter { s: self }
    }
}

impl<'s> Iterator for Iter<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_start();
        let fst = self.s.chars().next()?;
        let end = match fst {
            '"' => self.s[1..].find('"').map(|i| i + 2).unwrap_or(self.s.len()),
            '[' => {
                let mut i = 1;
                self.s[1..]
                    .find(|c: char| {
                        match c {
                            ']' if i == 1 => return true,
                            ']' => i -= 1,
                            '[' => i += 1,
                            _ => (),
                        }
                        false
                    })
                    .map(|i| i + 2)
                    .unwrap_or(self.s.len())
            }
            '{' => {
                let mut i = 1;
                self.s[1..]
                    .find(|c: char| {
                        match c {
                            '}' if i == 1 => return true,
                            '}' => i -= 1,
                            '{' => i += 1,
                            _ => (),
                        }
                        false
                    })
                    .map(|i| i + 2)
                    .unwrap_or(self.s.len())
            }
            _ => self.s.find(char::is_whitespace).unwrap_or(self.s.len()),
        };
        let r = &self.s[..end];
        self.s = &self.s[end..];
        Some(r)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tokens() {
        assert_eq!(
            r#"1 2 c 4.3 [ 1 2 3 ] [ [ [ 1 ] ] ] "ola amigos tudo bem" "#
                .split_tokens()
                .collect::<Vec<_>>(),
            vec![
                "1",
                "2",
                "c",
                "4.3",
                "[ 1 2 3 ]",
                "[ [ [ 1 ] ] ]",
                r#""ola amigos tudo bem""#
            ]
        )
    }
}
