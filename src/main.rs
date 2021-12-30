use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read string from stdin man get some help");

    count(&input)
}

fn count(input: &String) {
    let mut previous = ' ';
    let mut counter = 1;
    let mut winner = 1;
    let mut char_winner = ' ';

    for (index, current_char) in input.chars().enumerate() {
        if index == 0 {
            previous = current_char;
            continue;
        }

        if previous == current_char {
            counter += 1;
        } else {
            if winner < counter {
                winner = counter;
                char_winner = previous;
            }

            counter = 1;
        }

        previous = current_char;
    }

    if winner < counter {
        winner = counter;
        char_winner = previous;
    }

    for _i in 0..winner {
        print! {"{}", char_winner}
    }
}
