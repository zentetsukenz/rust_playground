fn main() {
    for num in 1..100 {
        let answer =
            if div_by_fifteen(num) {
                "FizzBuzz".to_string()
            }
            else if div_by_three(num) {
                "Fizz".to_string()
            }
            else if div_by_five(num) {
                "Buzz".to_string()
            }
            else {
                num.to_string()
            };

        println!("{}", answer);
    }
}

fn div_by_three(num: isize) -> bool {
    num % 3 == 0
}

fn div_by_five(num: isize) -> bool {
    num % 5 == 0
}

fn div_by_fifteen(num: isize) -> bool {
    num % 15 == 0
}

#[test]
fn test_div_by_three() {
    if div_by_three(1) {
        panic!("One is not three");
    }
}

#[test]
fn test_div_by_three_with_three() {
    if !div_by_three(3) {
        panic!("Three should be there");
    }
}

#[test]
fn test_div_by_fifteen() {
    if !div_by_three(15) {
        panic!("Fifteen should be there");
    }
}
