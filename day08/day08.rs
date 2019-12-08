use std::io;

const HEIGHT: usize = 6;
const WIDTH: usize = 25;
const LAYER_SIZE: usize = HEIGHT * WIDTH;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().as_bytes();

    let mut min_zeros = std::usize::MAX;
    let mut ones_times_twos = 0;

    let mut image: Vec<Vec<u8>> = (0..HEIGHT)
        .map(|_| (0..WIDTH).map(|_| '2' as u8).collect())
        .collect();

    let mut i = 0;

    while i < input.len() {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;

        for j in 0..LAYER_SIZE {
            match input[i + j] as char {
                '0' => zeros += 1,
                '1' => ones += 1,
                '2' => twos += 1,
                _ => (),
            }

            let row = j / WIDTH;
            let col = j % WIDTH;
            if image[row][col] as char == '2' {
                image[row][col] = input[i + j];
            }
        }

        if zeros < min_zeros {
            min_zeros = zeros;
            ones_times_twos = ones * twos;
        }

        i += LAYER_SIZE;
    }

    println!("part 1: {}", ones_times_twos);
    println!("part 2:");
    for row in image {
        for col in row {
            match col as char {
                '0' => print!(" "),
                '1' => print!("#"),
                '2' => print!("~"),
                _ => (),
            }
        }
        println!();
    }
}
