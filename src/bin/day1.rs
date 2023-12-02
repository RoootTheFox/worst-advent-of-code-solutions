use std::fmt::Write;
fn main() {
    let part2 = true;
    let input = include_str!("../../day1/input").to_string();

    let pairs: Vec<u16> = input
        .split('\n')
        .filter_map(|inp| {
            let a: u32;
            let b: u32;
            if part2 {
                let mut results = (0..inp.len()).filter_map(|i| {
                    let line_starting = &inp[i..];

                    if line_starting.starts_with("zero") {
                        Some(0)
                    } else if line_starting.starts_with("one") {
                        Some(1)
                    } else if line_starting.starts_with("two") {
                        Some(2)
                    } else if line_starting.starts_with("three") {
                        Some(3)
                    } else if line_starting.starts_with("four") {
                        Some(4)
                    } else if line_starting.starts_with("five") {
                        Some(5)
                    } else if line_starting.starts_with("six") {
                        Some(6)
                    } else if line_starting.starts_with("seven") {
                        Some(7)
                    } else if line_starting.starts_with("eight") {
                        Some(8)
                    } else if line_starting.starts_with("nine") {
                        Some(9)
                    } else {
                        // so i spent like 20 minutes figuring out why this was returning 'e` for two1nine
                        // until my brain realized i wasn't checking if its a number and just blindly returning
                        // a char lmao. i live in a fucking cave
                        // filter_map clears :3
                        line_starting.chars().nth(0)?.to_digit(10)
                    }
                });
                a = results.next()?;
                b = results.last().unwrap_or(a);
            } else {
                let nums: Vec<&str> = inp
                    .matches(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
                    .collect();
                a = nums.first()?.parse().ok()?;
                b = nums.last()?.parse().ok()?;
            }

            let mut number = String::with_capacity(4);
            write!(&mut number, "{}", a).ok()?;
            write!(&mut number, "{}", b).ok()?;

            u16::from_str_radix(&*number, 10).ok()
        })
        .collect();
    let mut result: u32 = 0;
    pairs.iter().for_each(|p| {
        result += *p as u32;
    });
    println!("=> {result}");
}
