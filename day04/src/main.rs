fn get_first_24_bits_of_md5(key: &str, index: usize) -> u32 {
    let digest = *md5::compute(format!("{}{}", key, index));
    ((digest[0] as u32) << 16) + ((digest[1] as u32) << 8) + digest[2] as u32
}

fn part_1(key: &str) -> usize {
    (0..)
        .take_while(|i| get_first_24_bits_of_md5(key, *i) & 0x00FF_FFF0 != 0)
        .count()
}

fn part_2(key: &str) -> usize {
    (0..)
        .take_while(|i| get_first_24_bits_of_md5(key, *i) & 0x00FF_FFFF != 0)
        .count()
}

fn main() {
    const KEY: &str = "bgvyzdsv";

    assert_eq!(254_575, part_1(KEY));
    assert_eq!(1_038_736, part_2(KEY));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(609_043, part_1("abcdef"));
        assert_eq!(1_048_970, part_1("pqrstuv"));
    }
}
