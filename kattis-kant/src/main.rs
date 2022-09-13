use std::io;

fn main() {
    // get standard input stream
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed read 1");
    let rectsize: Vec<usize> = input // 0 is number of rows(y) , 1 is number of columns(x)
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Could not parse input"))
        .collect();

    // prints the answer
    for x in 1..(rectsize[0] + 1) {
        for y in 1..(rectsize[1] + 1) {
            print!("{}", fastest_way(x, y, &rectsize));
        }
        print!("\n");
    }
}
// returns the least steps and if more than 9 return "."
fn fastest_way(mut x: usize, mut y: usize, size:&Vec<usize>) -> String {
    if x > (size[0] / 2) { // if x is on right side
        x = size[0] - x + 1; // count from right side
    }
    if y > (size[1] / 2) { // om y is under the middle
        y = size[1] - y + 1; // count from bottom
    }
    if x <= y { // if x is smaller than y or equal to
        if x > 9 { // if too big
            return ".".to_string();
        }
        return x.to_string();
    }
    if y > 9 { // if too big
        return ".".to_string();
    }
    return y.to_string();
}
