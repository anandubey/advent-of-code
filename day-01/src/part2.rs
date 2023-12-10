#[derive(Debug, Default)]
struct TrieNode {
    is_end_char: bool,
    children: HashMap<char, TrieNode>,
}

struct Trie {
    root: TrieNode,
}
pub fn process(input: &str) -> String {
    let alphabet_size = 26;

    let d: u32 = input
        .split("\n")
        .filter_map(|inp| {

            // inp.chars().find(|ch| ch.is_numeric()).map(|left| {
            //     ((left as u32) - 48) * 10
            //         + ((inp.chars().rfind(|ch| ch.is_numeric()).unwrap() as u32) - 48)
            // })
        })
        .sum();
    d.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
    ";
        assert_eq!("281", process(input));
    }
}
