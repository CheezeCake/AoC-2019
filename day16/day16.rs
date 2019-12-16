use std::io;

fn fft(input: &Vec<u8>) -> Vec<u8> {
    let mut next_phase = input.clone();
    let pattern = [0i32, 1, 0, -1];

    for r in 1..=input.len() {
        let mut pi = 0;
        let mut n = r - 1;
        let mut sum: i32 = 0;
        for i in 0..input.len() {
            if n == 0 {
                n = r;
                pi = (pi + 1) % pattern.len();
            }
            sum += input[i] as i32 * pattern[pi];
            n -= 1;
        }

        next_phase[r - 1] = (sum.abs() % 10) as u8;
    }

    next_phase
}

fn extract_message(digits: Vec<u8>, offset: usize) -> String {
    String::from_utf8(
        digits
            .get(offset..offset + 8)
            .unwrap()
            .iter()
            .map(|d| d + '0' as u8)
            .collect(),
    )
    .unwrap()
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).unwrap();
    let input: Vec<u8> = input_str
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    println!(
        "part 1: {}",
        extract_message(
            (0..100).fold(input.clone(), |digits, _| fft(&digits).clone()),
            0
        )
    );

    let msg_offset: usize = input_str[0..7].parse().unwrap();
    let mut digits = vec![];
    for _ in 0..10_000 {
        digits.append(&mut input.clone());
    }

    for _ in 0..100 {
        let mut sum = 0u32;
        for i in (msg_offset..digits.len()).rev() {
            sum += digits[i] as u32;
            digits[i] = (sum % 10) as u8;
        }
    }

    println!("part 2: {}", extract_message(digits, msg_offset));
}
