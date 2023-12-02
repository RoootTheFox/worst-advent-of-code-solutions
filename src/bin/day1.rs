use std::fmt::Write;
fn main() {
    let part2 = true;
    let input = include_str!("../../day1/input").to_string();

    let pairs: Vec<u16> = input.split('\n').map(|inp| {
        if inp.len() == 0 {
            return 0;
        }

        let a:u32;
        let b:u32;
        if part2 {
            let mut results = (0..inp.len()).filter_map(|i| {
                let line_starting = &inp[i..];

                if line_starting.starts_with("zero") {
                    //println!("zero {}", i);
                    Some(0)
                } else if line_starting.starts_with("one") {
                    //println!("one {}", i);
                    Some(1)
                } else if line_starting.starts_with("two") {
                    //println!("two {}", i);
                    Some(2)
                } else if line_starting.starts_with("three") {
                    //println!("three {}", i);
                    Some(3)
                } else if line_starting.starts_with("four") {
                    //println!("four {}", i);
                    Some(4)
                } else if line_starting.starts_with("five") {
                    //println!("five {}", i);
                    Some(5)
                } else if line_starting.starts_with("six") {
                    //println!("six {}", i);
                    Some(6)
                } else if line_starting.starts_with("seven") {
                    //println!("seven {}", i);
                    Some(7)
                } else if line_starting.starts_with("eight") {
                    //println!("eight {}", i);
                    Some(8)
                } else if line_starting.starts_with("nine") {
                    //println!("nine {}", i);
                    Some(9)
                } else {
                    // so i spent like 20 minutes figuring out why this was returning 'e` for two1nine
                    // until my brain realized i wasn't checking if its a number and just blindly returning
                    // a char lmao. i live in a fucking cave
                    // filter_map clears :3
                    line_starting.chars().nth(0).unwrap().to_digit(10)
                }
            });
            a = results.next().unwrap();
            b = results.last().unwrap_or(a);
        } else {
            let nums: Vec<&str> = inp.matches(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']).collect();
            let a_str = nums.first().unwrap();
            a = a_str.parse().unwrap();
            b = nums.last().unwrap_or(a_str).parse().unwrap();
        }

        //println!("{}", inp);
        //println!("a: {} b: {}", a, b);

        let mut number = String::with_capacity(4);
        write!(&mut number, "{}", a).expect("moyai emoji");
        write!(&mut number, "{}", b).expect("moyai emoji");


        u16::from_str_radix(&*number, 10).unwrap()
    }).collect();
    let mut result:u32 = 0;
    pairs.iter().for_each(|p| {
        result += *p as u32;
    });
    println!("day 1 {}", result);
}
