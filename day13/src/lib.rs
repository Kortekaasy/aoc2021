mod main;

pub fn part1() -> String {
    let (field, folds) = main::parse_input(include_str!("../input").to_owned());
    main::part1(field, folds)
}

pub fn part2() -> String {
    let (field, folds) = main::parse_input(include_str!("../input").to_owned());
    main::part2(field, folds)
}