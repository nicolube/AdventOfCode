use std::fs;

#[tokio::main]
async fn main() {
    let doc1 = fs::read_to_string("document1.txt")
        .expect("Something went wrong reading the file");
    println!("Result: {}", get_result1(&doc1));

    let doc2 = fs::read_to_string("document2.txt")
        .expect("Something went wrong reading the file");
    println!("Result: {}", get_result1(&doc2));

    let doc3 = fs::read_to_string("document3.txt").expect("Something went wrong reading the file");
    println!("Result: {}", get_result2(&doc3));

    println!("Result: {}", get_result2(&doc2));
}

fn get_result1(input: &String) -> i32 {
    let mut result: i32 = 0;
    for line in input.lines() {
        let a: Option<usize> = line.find(char::is_numeric);
        let b: Option<usize> = line.rfind(char::is_numeric);

        let mut a_num: i32 = 0;
        let mut b_num: i32 = 0;

        if let Some(a) = a {
            a_num = line[a..a + 1].parse::<i32>().unwrap();
        }
        if let Some(b) = b {
            b_num = line[b..b + 1].parse::<i32>().unwrap();
        }

        result += a_num * 10 + b_num;
    }

    return result;
}

static VALIED_STRING_NUMBERS: &'static [&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_result2(input: &String) -> i32 {
    let mut result: i32 = 0;
    for line in input.lines() {
        let a: Option<usize> = line.find(char::is_numeric);
        let b: Option<usize> = line.rfind(char::is_numeric);


        let mut a_num: i32 = 0;
        let mut b_num: i32 = 0;
        if let Some(a) = a {
            a_num = line[a..a + 1].parse::<i32>().unwrap();
        }
        if let Some(b) = b {
            b_num = line[b..b + 1].parse::<i32>().unwrap();
        }

        let mut a = a.unwrap_or(usize::MAX);
        let mut b = b.unwrap_or(usize::MIN);

        for i in 0..VALIED_STRING_NUMBERS.len() {
            let find_a = line.find(VALIED_STRING_NUMBERS[i]);
            let find_b = line.rfind(VALIED_STRING_NUMBERS[i]);
            if let Some(aa) = find_a {
                if aa < a {
                    a = aa;
                    a_num = i as i32 + 1;
                }
            }
            if let Some(bb) = find_b {
                if bb > b {
                    b = bb;
                    b_num = i as i32 + 1;
                }
            }
        }

        result += a_num * 10 + b_num;
    }

    return result;
}
