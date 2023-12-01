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
