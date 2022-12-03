use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("It should work bro");
    let parsed = parse(content);
    println!("{:?}", parsed)
}

// temp; if != "": counter += line, else: if temp <= counter: temp = counter;
// While the line isn't blank, we add up parsed numbers to the counter. If the counter is higher than the
// current temporary variable, then we replace it's value by the value of the counter.
fn parse(input: String) -> i64 {
    let mut temp: i64 = 0;
    let mut counter: i64 = 0;

    for line in input.lines() {
        if line != "" {
            let num = line.parse::<i64>().unwrap();
            counter += num;
        } else {
            if temp <= counter {
                temp = counter;
            }
            counter = 0;
        }
    }
    temp
}
