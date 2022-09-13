use std::io;

fn main() {
    // get standard input stream
    let mut i1 = String::new();
    let mut i2 = String::new();

    // reads line
    io::stdin().read_line(&mut i1).expect("Failed read 1");
    io::stdin().read_line(&mut i2).expect("Failed read 2");

    let mut first_number: usize = i1.trim().parse().expect("Could not parse 1st number"); // parses first number
    if first_number % 2 == 1 { // adds 1 if not even
        first_number += 1;
    }

    let mut numbers: Vec<u32> = i2 // makes vec of the numbers
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Could not parse numbers!"))
        .collect();
    numbers.sort(); //sort in lowest to highest
    numbers.reverse(); // make it highest to lowest

    let mut answer: u32 = 0;
    for _index in 0..(first_number / 2) {
        answer += numbers[_index]; // adds for every number
    }

    println!("{}", answer);
}
