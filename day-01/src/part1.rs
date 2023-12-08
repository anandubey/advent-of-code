pub fn process(input: &str) -> String {
    let d: u32 = input
        .split("\n")
        .filter_map(|inp| {
            inp.chars().find(|ch| ch.is_numeric()).map(|left| {
                ((left as u32) - 48) * 10
                    + ((inp.chars().rfind(|ch| ch.is_numeric()).unwrap() as u32) - 48)
            })
        })
        .sum();
    d.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        ";
        assert_eq!("142", process(input));
    }
}
