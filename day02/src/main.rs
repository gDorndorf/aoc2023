use std::cmp;
use std::fs;

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("./input.txt").unwrap();

    let lines = input.lines();
    let data = lines.map(String::from).collect::<Vec<String>>();

    let red = 12;
    let green = 13;
    let blue = 14;

    let res: i32 = data.iter().fold(0, |res, line| {
        let line: Vec<&str> = line.split(":").collect();
        let (no, game) = (
            line[0].split(" ").nth(1).expect("").parse::<i32>().unwrap(),
            line[1],
        );
        let legal = game.split(";").fold(1, |valid, game| {
            valid * {
                game.split(",").fold(1, |valid, cubes| {
                    let cubes: Vec<&str> = cubes.split(" ").collect();
                    let (amount, color) = (cubes[1], cubes[2]);
                    let amount = amount.parse::<i32>().unwrap();
                    let max_amount = match color {
                        "red" => red,
                        "blue" => blue,
                        "green" => green,
                        &_ => 0,
                    };
                    valid * (amount <= max_amount) as i32
                })
            }
        });
        res + no * legal
    });
    print!("{:?}\n", res);

    let res: i32 = data.iter().fold(0, |res, line| {
        let line: Vec<&str> = line.split(":").collect();
        let (no, game) = (
            line[0].split(" ").nth(1).expect("").parse::<i32>().unwrap(),
            line[1],
        );
        let minimum = game.split(";").fold((0, 0, 0), |min, game| {
            {
                game.split(",").fold(min, |min, cubes| {
                    let cubes: Vec<&str> = cubes.split(" ").collect();
                    let (amount, color) = (cubes[1], cubes[2]);
                    let amount = amount.parse::<i32>().unwrap();
                    match color {
                        "red" => (std::cmp::max(min.0, amount), min.1, min.2),
                        "green" => (min.0, std::cmp::max(min.1, amount), min.2),
                        "blue" => (min.0, min.1, std::cmp::max(min.2, amount)),
                        &_ => (0, 0, 0),
                    }
                })
            }
        });
        res + minimum.0 * minimum.1 * minimum.2
    });
    print!("{:?}\n", res)
}
