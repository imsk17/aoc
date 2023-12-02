pub mod part1 {

    const INPUT: &str = include_str!("../../inputs/day2/part1.txt");

    pub fn solution() -> usize {
        let result = INPUT
            .lines()
            .map(|e| {
                e.split_once(": ")
                    .unwrap()
                    .1
                    .split(";")
                    .map(|s| s.split(", ").map(|s| s.trim_start()).collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .enumerate()
            .map(|(index, data)| {
                let gid = index + 1;
                let should_not_count = data
                    .iter()
                    .flatten()
                    .map(|e| e.split_once(" ").unwrap())
                    .any(|(count, color)| {
                        let count = count.parse::<u8>().unwrap();
                        match (color, count) {
                            ("red", i) if i > 12 => return true,
                            ("green", i) if i > 13 => return true,
                            ("blue", i) if i > 14 => return true,
                            _ => false,
                        }
                    });
                if should_not_count {
                    return 0;
                }
                gid
            })
            .sum();

        result
    }
}
