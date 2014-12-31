use std::io;
use std::rand;

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");

    let secret_number = (rand::random::<uint>() % 100u) + 1u;

    loop {
      println!("Please input your guess");

      let input = io::stdin().read_line()
                             .ok()
                             .expect("failed to read line");

      let input_num: Option<uint> = input.trim().parse();
      println!("{}", input_num);

      let num = match input_num {
        Some(num) => num,
        None => {
          println!("Please input a number!");
          continue;
        }
      };

      println!("You guessed: {}", input);

      match cmp(num, secret_number) {
        Less => println!("Too small!"),
        Greater => println!("Too big!"),
        Equal => {
          println!("You Win!");
          return
        }
      }
    }
}

fn cmp(a: uint, b: uint) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

