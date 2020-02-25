use serde_json::{Result, Value};
use std::fs;

fn part_1(v: &Value) -> i64 {
    let mut sum = 0;
    match v {
        Value::Number(number) => sum += number.as_i64().unwrap(),
        Value::Array(array) => sum += array.iter().map(|value| part_1(value)).sum::<i64>(),
        Value::Object(object) => sum += object.values().map(|value| part_1(value)).sum::<i64>(),
        _ => (),
    };

    sum
}

fn part_2(v: &Value) -> i64 {
    let mut sum = 0;
    match v {
        Value::Number(number) => sum += number.as_i64().unwrap(),
        Value::Array(array) => sum += array.iter().map(|value| part_2(value)).sum::<i64>(),
        Value::Object(object) => {
            let no_red = !object.values().any(|value| {
                if let Value::String(s) = value {
                    return s == "red";
                }
                false
            });

            if no_red {
                sum += object.values().map(|value| part_2(value)).sum::<i64>();
            }
        }
        _ => (),
    };

    sum
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let v: Value = serde_json::from_str(input)?;

    assert_eq!(156_366, part_1(&v));
    assert_eq!(96_852, part_2(&v));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        assert_eq!(6, part_1(&serde_json::from_str(r#"[1,2,3]"#)?));
        assert_eq!(6, part_1(&serde_json::from_str(r#"{"a":2,"b":4}"#)?));
        assert_eq!(3, part_1(&serde_json::from_str(r#"[[[3]]]"#)?));
        assert_eq!(3, part_1(&serde_json::from_str(r#"{"a":{"b":4},"c":-1}"#)?));
        assert_eq!(0, part_1(&serde_json::from_str(r#"{"a":[-1,1]}"#)?));
        assert_eq!(0, part_1(&serde_json::from_str(r#"[-1,{"a":1}]"#)?));
        assert_eq!(0, part_1(&serde_json::from_str(r#"[]"#)?));
        assert_eq!(0, part_1(&serde_json::from_str(r#"{}"#)?));

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        assert_eq!(6, part_2(&serde_json::from_str(r#"[1,2,3]"#)?));
        assert_eq!(4, part_2(&serde_json::from_str(r#"[1,{"c":"red","b":2},3]"#)?));
        assert_eq!(0, part_2(&serde_json::from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)?));
        assert_eq!(6, part_2(&serde_json::from_str(r#"[1,"red",5]"#)?));

        Ok(())
    }
}
