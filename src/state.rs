use std::fmt::Display;

/// status of a game round.
#[derive(Debug)]
pub struct State {
    guess: Vec<char>,
    target: String,
}

impl State {
    pub fn from_word(target: String) -> Self {
        let guess = target
            .chars()
            // replace non-alphabetics to '_'
            .map(|ch| if ch.is_alphabetic() { '_' } else { ch })
            .collect();
        Self { guess, target }
    }

    /// assumes guess_try is never empty.
    pub fn guess(&mut self, guess_try: &str) -> bool {
        let mut guessed = false;
        match guess_try.len() {
            1 => {
                let guess_char = guess_try.chars().next().unwrap();
                for (i, current_char) in self.target.chars().enumerate() {
                    if current_char.to_ascii_lowercase() == guess_char.to_ascii_lowercase() {
                        self.guess[i] = current_char;
                        guessed = true;
                    }
                }
            }
            _ => {
                if self.target.to_lowercase() == guess_try.to_lowercase() {
                    self.guess = self.target.chars().collect();
                    guessed = true;
                }
            }
        }

        guessed
    }

    pub fn is_completed(&self) -> bool {
        self.target == self.guess.iter().collect::<String>()
    }

    pub fn target_word(&self) -> &str {
        &self.target
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.guess.iter().collect::<String>())?;
        Ok(())
    }
}
