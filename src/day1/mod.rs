const INPUT: &str = include_str!("../../inputs/day1/part1.txt");
pub mod part1 {
    pub fn solution() -> i32 {
        super::INPUT
            .lines()
            .map(|line| line.chars())
            .map(|chars| chars.filter(char::is_ascii_digit).collect::<Vec<_>>())
            .map(|v| match v.len() {
                0 | 1 => (v.first().cloned().unwrap(), v.first().cloned().unwrap()),
                _ => (v.first().cloned().unwrap(), v.last().cloned().unwrap()),
            })
            .map(|e| format!("{}{}", e.0, e.1).parse::<i32>().unwrap())
            .sum()
    }
}

pub mod part2 {
    pub fn solution() -> i32 {
        let sanitized = super::INPUT
            .replace("one", "o1ne")
            .replace("two", "t2wo")
            .replace("three", "th3ree")
            .replace("four", "fo4ur")
            .replace("five", "fi5ve")
            .replace("six", "s6ix")
            .replace("seven", "sev7en")
            .replace("eight", "ei8ght")
            .replace("nine", "ni9ne");
        sanitized
            .lines()
            .map(|line| line.chars())
            .map(|chars| chars.filter(char::is_ascii_digit).collect::<Vec<_>>())
            .map(|v| match v.len() {
                0 | 1 => (v.first().cloned().unwrap(), v.first().cloned().unwrap()),
                _ => (v.first().cloned().unwrap(), v.last().cloned().unwrap()),
            })
            .map(|e| format!("{}{}", e.0, e.1).parse::<i32>().unwrap())
            .sum()
    }
}
