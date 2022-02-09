use ferris_says::say;
use std::io::{stdout, BufWriter};

mod guessing_game;
use guessing_game::guess::guess_num;


fn main() {
    let std_out = stdout();
    let message = String::from("Hello rust world");
    let width = message.chars().count();

    let mut writer = BufWriter::new(std_out.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    guess_num();
}
