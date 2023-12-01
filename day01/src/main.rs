use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines = input.lines();
    let data = lines.map(String::from).collect::<Vec<String>>();
    print!("{}\n", part1(&data));
    print!("{}\n", part2(&data));
}

fn part2(input: &Vec<String>) -> u32 {
    let digits = vec![
        ("twone", 21),
        ("oneight", 18),
        ("threeight", 38),
        ("fiveeight", 58),
        ("nineight", 98),
        ("eightwo", 82),
        ("eighthree", 83),
        ("sevenine", 79),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let data: Vec<String> = input
        .iter()
        .map(|line| {
            digits
                .iter()
                .fold(line.clone(), |a, (d, i)| a.replace(d, &i.to_string()))
        })
        .collect();
    return part1(&data);
}

fn part1(input: &Vec<String>) -> u32 {
    let data = input.iter().fold(Vec::new(), |acc, line| {
        [
            acc,
            vec![
                (line
                    .chars()
                    .collect::<Vec<char>>()
                    .iter()
                    .fold(Vec::new(), |l, e| match e.to_digit(10) {
                        Some(i) => [l, vec![i]].concat(),
                        None => l,
                    })),
            ],
        ]
        .concat()
    });

    return data.iter().fold(0, |sum, v| {
        sum + 10 * v.first().unwrap() + v.last().unwrap()
    });
}
