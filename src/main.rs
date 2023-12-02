mod day1;
mod day2;

fn main() {
    let day1_part1 = day1::part1::solution();
    let day1_part2 = day1::part2::solution();

    let day2_part1 = day2::part1::solution();
    println!("Day 1, part 1: {}", day1_part1);
    println!("Day 1, part 2: {}", day1_part2);
    println!("Day 2, part 1: {}", day2_part1);
}
