fn main() {
    let part2 = true;
    let input = include_str!("../../day2/input").to_string();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let wow: u32 = input
        .split('\n')
        .filter_map(|inp| {
            let inp = inp.strip_prefix("Game ")?;

            // get le game :3c
            let game: Vec<&str> = inp.split(':').collect();
            let game_id = u32::from_str_radix(game[0], 10).ok()?;

            let mut p1_game_correct = true;
            let mut p2_min_red = 0;
            let mut p2_min_green = 0;
            let mut p2_min_blue = 0;

            // parse ROLL of game
            game[1].trim_start_matches(' ').split(";").for_each(|roll| {
                let mut roll_red = 0;
                let mut roll_green = 0;
                let mut roll_blue = 0;
                // parse the roll by splitting the results up
                roll.split(',').for_each(|mut res| {
                    res = res.trim_start_matches(' '); // we hate spaces!!

                    let split: Vec<&str> = res.split(' ').collect();
                    let num = u16::from_str_radix(split[0], 10).unwrap();
                    match split[1] {
                        "red" => {
                            if part2 {
                                roll_red = roll_red.max(num);
                            } else {
                                roll_red += num;
                            }
                        }
                        "green" => {
                            if part2 {
                                roll_green = roll_green.max(num);
                            } else {
                                roll_green += num;
                            }
                        }
                        "blue" => {
                            if part2 {
                                roll_blue = roll_blue.max(num);
                            } else {
                                roll_blue += num;
                            }
                        }
                        _ => {
                            println!("its [unknown]");
                        }
                    };
                });

                if part2 {
                    p2_min_red = p2_min_red.max(roll_red);
                    p2_min_green = p2_min_green.max(roll_green);
                    p2_min_blue = p2_min_blue.max(roll_blue);
                }
                if roll_red > max_red || roll_green > max_green || roll_blue > max_blue {
                    p1_game_correct = false;
                }
            });

            if part2 {
                return Some(p2_min_red as u32 * p2_min_green as u32 * p2_min_blue as u32);
            } else if !p1_game_correct {
                return None;
            }

            Some(game_id)
        })
        .sum();

    println!("=> {wow}");
}
