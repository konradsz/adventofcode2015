use std::fs;

fn decoded_len(input: &str) -> usize {
    let input = &input[1..input.len() - 1];
    let mut count = 0;
    let mut pos = 0;
    while pos < input.len() {
        if input.chars().nth(pos).unwrap() == '\\' {
            if input.chars().nth(pos + 1).unwrap() == 'x' {
                pos += 3;
            } else {
                pos += 1;
            }
        }

        pos += 1;
        count += 1;
    }

    count
}

fn encoded_len(input: &str) -> usize {
    let is_backslash_or_quote = |c: &char| *c == '\\' || *c == '"';
    let backslash_and_quote_count = input.chars().filter(is_backslash_or_quote).count();

    input.len() + backslash_and_quote_count + 2
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .fold(0, |acc, line| acc + (line.len() - decoded_len(line)))
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .fold(0, |acc, line| acc + (encoded_len(line) - line.len()))
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    assert_eq!(1350, part_1(content));
    assert_eq!(2085, part_2(content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoded_len() {
        assert_eq!(0, decoded_len(r#""""#));
        assert_eq!(3, decoded_len(r#""abc""#));
        assert_eq!(7, decoded_len(r#""aaa\"aaa""#));
        assert_eq!(1, decoded_len(r#""\x27""#));
    }

    #[test]
    fn test_encoded_len() {
        assert_eq!(6, encoded_len(r#""""#));
        assert_eq!(9, encoded_len(r#""abc""#));
        assert_eq!(16, encoded_len(r#""aaa\"aaa""#));
        assert_eq!(11, encoded_len(r#""\x27""#));
    }
}
