extern crate md5;

fn get_first_24_bits_of_md5(key: &str, index: usize) -> u32 {
    let digest = *md5::compute(format!("{}{}", key, index));
    ((digest[0] as u32) << 16) + ((digest[1] as u32) << 8) + ((digest[2] as u32) << 0)
}

fn part_1(key: &str) -> usize {
    (0..)
        .take_while(|i| get_first_24_bits_of_md5(key, *i) & 0xFFFFF0 != 0)
        .count()
}

fn part_2(key: &str) -> usize {
    (0..)
        .take_while(|i| get_first_24_bits_of_md5(key, *i) & 0xFFFFFF != 0)
        .count()
}

fn main() {
    const KEY: &str = "bgvyzdsv";

    assert_eq!(254575, part_1(KEY));
    assert_eq!(1038736, part_2(KEY));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(609043, part_1("abcdef"));
        assert_eq!(1048970, part_1("pqrstuv"));
    }
}
