mod main;

pub fn part1() -> String {
    main::part1(&mut main::parse_input(include_str!("../input").to_owned()))
}

pub fn part2() -> String {
    main::part2(&mut main::parse_input(include_str!("../input").to_owned()))
}