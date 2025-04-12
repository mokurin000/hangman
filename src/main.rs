use std::io::stdin;

use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;

use hangman::init::read_words;
use hangman::state::State;
use hangman::{Result, MAX_FAILED_GUESS};

fn game_round(words: &[String], rng: &mut ThreadRng) -> Result<()> {
    let current_word = words
        .choose(rng)
        .expect("word list must be non-empty!")
        .clone();

    let mut game_state = State::from_word(current_word);
    let mut failure_times = 0;
    loop {
        let mut buf = String::new();

        println!();
        println!(
            "state: {game_state}, remaining: {}",
            MAX_FAILED_GUESS - failure_times
        );

        stdin().read_line(&mut buf)?;
        if !game_state.guess(buf.trim()) {
            println!("guess failed! ❌");
            failure_times += 1;
        }

        if game_state.is_completed() {
            break;
        }

        if failure_times == MAX_FAILED_GUESS {
            break;
        }
    }

    if game_state.is_completed() {
        println!("~~~ 🎉 Congratulations! ~~~");
    } else {
        println!("~~~ 😢 Oops, out of tries! ~~~");
    }
    println!("Target word: {}", game_state.target_word());

    println!("----- ROUND END! -----");
    println!();

    Ok(())
}

fn main() -> Result<()> {
    let words = read_words()?;
    let mut rng = ThreadRng::default();
    loop {
        game_round(&words, &mut rng)?;

        println!("Do you want to play next round? (y/n)");
        let mut confirm = String::new();
        stdin().read_line(&mut confirm)?;
        if !confirm.to_lowercase().starts_with("y") {
            println!("See you again!");
            break;
        }
    }
    Ok(())
}
