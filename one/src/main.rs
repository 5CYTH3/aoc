use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("It should work bro");
    let higher_calories = calculate_higher_calories(content.clone());
    println!("{}", higher_calories);

    let top_three = calculate_top_three(content);

    println!("{:?}", top_three);
}

// temp; if != "": counter += line, else: if temp <= counter: temp = counter;
// While the line isn't blank, we add up parsed numbers to the counter. If the counter is higher than the
// current temporary variable, then we replace it's value by the value of the counter.
fn calculate_higher_calories(input: String) -> i64 {
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

fn calculate_top_three(input: String) -> (i64, i64, i64) {
    let mut third: i64 = 0;
    let mut second: i64 = 0;
    let mut first: i64 = 0;
    let mut counter: i64 = 0;

    for line in input.lines() {
        if line != "" {
            let num = line.parse::<i64>().unwrap();
            counter += num;
        } else {
            if first <= counter {
                third = second;
                second = first;
                first = counter;
            }
            counter = 0;
        }
    }

    (first, second, third)
}
