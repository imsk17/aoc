pub mod part1 {

    const INPUT: &str = include_str!("../../inputs/day1/part1.txt");

    pub fn solution() -> u32 {
        let mut acc = 0;

        for line in INPUT.lines() {
            let mut digits_in_line = line.chars().filter(|c| c.is_digit(10));
            let [first, .., last] = if digits_in_line.clone().count() < 2 {
                [
                    digits_in_line.clone().next().unwrap(),
                    digits_in_line.clone().next().unwrap(),
                ]
            } else {
                [
                    digits_in_line.next().unwrap(),
                    digits_in_line.last().unwrap_or('0'),
                ]
            };
            let digit = format!("{}{}", first, last).parse::<u32>().unwrap();
            acc += digit;
        }
        return acc;
    }
}

pub mod part2 {
    const INPUT: &str = include_str!("../../inputs/day1/part1.txt");

    pub fn solution() -> u32 {
        let mut acc = 0;
        let sanitized = INPUT
            .replace("one", "o1ne")
            .replace("two", "t2wo")
            .replace("three", "th3ree")
            .replace("four", "fo4ur")
            .replace("five", "fi5ve")
            .replace("six", "s6ix")
            .replace("seven", "sev7en")
            .replace("eight", "ei8ght")
            .replace("nine", "ni9ne");

        for line in sanitized.lines() {
            let mut digits_in_line = line.chars().filter(|c| c.is_digit(10));
            let [first, .., last] = if digits_in_line.clone().count() < 2 {
                [
                    digits_in_line.clone().next().unwrap(),
                    digits_in_line.clone().next().unwrap(),
                ]
            } else {
                [
                    digits_in_line.next().unwrap(),
                    digits_in_line.last().unwrap_or('0'),
                ]
            };
            let digit = format!("{}{}", first, last).parse::<u32>().unwrap();
            acc += digit;
        }
        return acc;
    }
}
