## Phase 1: Load data

We know `fruits.txt` contains words to guess. And to dedup them, we could import `AHashSet` from `ahash`, (ahash performs best for strings).

## Phase 2: State

As the goal of hangman is to guess all letters from a word, we record current state as a `Vec<char>`, to easily modify each char inside it. The `fruits.txt` does not contains any non-ASCII characters though, the design could make the game mostly unicode ready.

Also, we implemented serveral methods on the `State` structure:

- `from_word` constructs new blank `State` with a target word, leave non-alphabetics unchanged, otherwise replace with underscore (`_`) in the `Vec<char>`.
  - We could call `str::chars` for `target`, to iterate over chracters inside it. Meanwhile, we could determine each character by `char::is_alphabetic`.
- `guess` accepts a `&str` argument `guess_try`.
  - returns a boolean, representing success/failure of a guess.
  - First, we `match` length of `guess_try`. If it's of single character, we handle this as single alpha guess: iterate over `self.target` and get corresponding index by `enumerate` method on `Chars`. Otherwise, we should determine `&self.target == guess_try`, validation of whole-word guess here is much more straight forward.
  - To make guesses case-insentive, we could call `char::to_ascii_lowercase` and `str::to_lowercase`, before equality checks.
- `is_completed` checks whether the user guessed all letters.
  - We could just `collect` an iterator of `char`'s from `self.guess` to a single `String`, and check if it equals to the target word.
- `target_word` returns target word of this game round.

To display the `State`, we simply implement `Display` for the `State`.

## Phase 3: Interative UI

### Random choose of words

`rand` crate provides `choose` method for slices, so we call `words.choose(&mut rng)` to get an random word.

### Get input from stdin

In Rust, we don't have unsafe `scanf` like in C. To get a line from standard input, we must call `std::io::stdin()`, besides `.read_line(buffer)`, where buffer is a mutable borrow for `String`.
Performance is not really important here, creates the `String` inside loop is fine. Otherwise we must call `String::clear` before inputs, as `read_line` will never remove old contents.

### UI

We want to allow users input guesses repeatly, so we should handle user input in a `loop`.
The loop will `break` if the `State::is_completed` returns `true`.

For the rule `FM`, we define a constant `MAX_FAILED_GUESS`. When an user performed a failed guess, we will bump `failure_times` by 1.
Once it hits `MAX_FAILED_GUESS`, the loop is over, the game is also over.

To tell users their remaining failed attempts (RA), we can substract `MAX_FAILED_GUESS` by `failure_times`.

Once the loop exits, we could check `game_state.is_completed()`, to determine it's succession/failure. This approach, we need not to care final status of the game inside `loop`.

Last, to allow continous play (the rule SQ), we place the whole round logic to a function `game_round`, call the function inside a simpler loop in the `main()` function. We could check confirmation of user by `.to_lowercase().startswith("y")`
