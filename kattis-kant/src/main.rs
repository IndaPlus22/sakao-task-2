use std::io;

fn main() {
    // get standard input stream
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed read 1");
    let rectsize: Vec<usize> = input
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Could not parse input"))
        .collect();

    for x in 1..(rectsize[0] + 1) {
        for y in 1..(rectsize[1] + 1) {
            print!("{}", fastest_way(x, y, &rectsize));
        }
        print!("\n");
    }
    // println!("{}", answer);
}

fn fastest_way(mut x: usize, mut y: usize, size:&Vec<usize>) -> String {
    if x > (size[0] / 2) {
        x = size[0] - x + 1;
    }
    if y > (size[1] / 2) {
        y = size[1] - y + 1;
    }
    if x <= y {
        if x > 9 {
            return ".".to_string();
        }
        return x.to_string();
    }
    if y > 9 {
        return ".".to_string();
    }
    return y.to_string();
}
