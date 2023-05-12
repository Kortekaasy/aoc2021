mod main;

pub fn part1() -> String {
    let (template, rules) = main::parse_input(include_str!("../input").to_owned());
    main::part1(&template, &rules)
}

pub fn part2() -> String {
    let (template, rules) = main::parse_input(include_str!("../input").to_owned());
    main::part2(&template, &rules)
}