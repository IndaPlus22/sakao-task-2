use std::{io, vec};

fn main() {
    // get standard input stream
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed read 1");
    let mut rectsize: Vec<usize> = input
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Could not parse input"))
        .collect();

    for y in 1..(rectsize[1] + 1) {
        for x in 1..(rectsize[0] + 1) {
            eprint!("{}", fastest_way(x, y, &rectsize));
        }
        eprint!("\n");
    }

    

    // println!("{}", answer);
}

fn fastest_way(mut x: usize, mut y: usize, size:&Vec<usize>) -> u32 {
    //check x and y and compare which is smallest and return smallest

    // (4 - 5) + 1= 2
    if x > (size[0] / 2) {
        x = size[0] - x + 1;
    }
    if y > (size[1] / 2) {
        y = size[1] - y + 1;
    }
    if x <= y {
        return x as u32;
    }
    return y as u32;
}
