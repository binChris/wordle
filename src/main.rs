//! A small helper to solve wordle puzzles.
//!
//! The app list all words that match the filter.
//!
//! The filter can be modified with the following keyboard shortcuts:
//! - `+` for 'character must occur'
//! - `-` for 'must not occur'
//! - `1-5` for 'must be in position'
//! - `esc` or `*` for any position
//! - any character to apply the chosen filter

use anyhow::Result;
use crossterm::{
    event::{self, KeyEvent, KeyEventKind},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::{fs::read_to_string, io::stdout, path::Path};

const WORD_LENGTH: usize = 5;
/*
Different filter types:
- occurence, char must occur (possibly multiple times), or must not occur
- positional, must be x or must not be x,y,z
  - if must be x, existing 'must not be' filter can discarded
  */
#[derive(Debug, Clone)]
enum PositionalFilter {
    MustBe(char),
    MustNotBe(Vec<char>),
}

#[derive(Debug, Clone)]
struct Filter {
    positional: Vec<Option<PositionalFilter>>,
    must_occur: Vec<char>,
    must_not_occur: Vec<char>,
}

impl Filter {
    fn print(&self) {
        let mut lines = vec![];
        for (i, p) in self.positional.iter().enumerate() {
            match p {
                Some(PositionalFilter::MustBe(ch)) => {
                    lines.push(format!("- char {} must be {}", i + 1, ch));
                }
                Some(PositionalFilter::MustNotBe(chars)) => {
                    lines.push(format!("- char {} must not be {:?}", i + 1, chars));
                }
                None => {}
            }
        }
        if !self.must_occur.is_empty() {
            lines.push(format!("- word must contain: {:?}", self.must_occur));
        }
        if !self.must_not_occur.is_empty() {
            lines.push(format!(
                "- word must not contain: {:?}",
                self.must_not_occur
            ));
        }
        if !lines.is_empty() {
            println!("Filter:\n{}", lines.join("\n"));
        }
    }

    fn matches(&self, word: &str) -> bool {
        for (i, c) in word.chars().enumerate() {
            match self.positional[i] {
                Some(PositionalFilter::MustBe(ch)) => {
                    if c != ch {
                        return false;
                    }
                }
                Some(PositionalFilter::MustNotBe(ref chars)) => {
                    if chars.contains(&c) {
                        return false;
                    }
                }
                None => {}
            }
        }
        let mut w = word.to_string();
        for c in &self.must_occur {
            match w.find(*c) {
                None => return false,
                Some(i) => {
                    // remove the matched character to properly match multiple identical characters
                    w.replace_range(i..i + 1, "");
                }
            }
        }
        // apply the 'must not occur' filter only to characters that don't have a positional 'must be'
        let masked_word = self
            .positional
            .iter()
            .enumerate()
            .filter(|(_, p)| !matches!(p, Some(PositionalFilter::MustBe(_))))
            .map(|(i, _)| word.chars().nth(i).unwrap())
            .collect::<String>();
        for c in &self.must_not_occur {
            if masked_word.contains(*c) {
                return false;
            }
        }
        true
    }

    fn is_empty(&self) -> bool {
        self.positional.iter().all(|p| p.is_none())
            && self.must_occur.is_empty()
            && self.must_not_occur.is_empty()
    }
}

// InputMode defines how character filters are applied:
enum InputMode {
    // Positional: in position x character must be c (true) or must not be c (false)
    Positional(usize, bool),
    // Global: character must occur (true) or must not occur (false)
    Global(bool),
}

impl InputMode {
    fn print(&self) {
        print!("Press any charactor to filter on ");
        match self {
            InputMode::Positional(x, true) => {
                println!("'position {} character must be'", x + 1);
            }
            InputMode::Positional(x, false) => {
                println!("'position {} character must not be'", x + 1);
            }
            InputMode::Global(true) => {
                println!("'word must contain'");
            }
            InputMode::Global(false) => {
                println!("'word must not contain'");
            }
        }
    }
}

const DEFAULT_INPUT_MODE: InputMode = InputMode::Global(false);

fn main() -> Result<()> {
    println!("Reading word list...");
    let words = read_words_from_file("words.txt", WORD_LENGTH)?;
    let mut filter = Filter {
        positional: vec![None; WORD_LENGTH],
        must_occur: vec![],
        must_not_occur: vec![],
    };
    let mut input_mode = DEFAULT_INPUT_MODE;
    loop {
        if filter.is_empty() {
            print_start_words();
        } else {
            print_word_list(&words, &filter, 10);
        }
        filter.print();
        println!("Press + for 'character must occur', - for 'must not occur', 1-5 for 'must be in position', esc for any position");
        input_mode.print();
        input_mode = process_input(input_mode, &mut filter);
    }
}

fn print_word_list(words: &[(String, bool)], filter: &Filter, max_words: usize) {
    let mut matches = vec![];
    let mut rare = vec![];
    for word in words {
        if filter.matches(&word.0) {
            if word.1 {
                matches.push(word);
            } else {
                rare.push(word);
            }
            if matches.len() >= max_words {
                break;
            }
        }
    }
    let matches: Vec<(String, bool)> = matches
        .iter()
        .chain(rare.iter())
        .take(max_words)
        .map(|w| (w.0.to_owned(), w.1))
        .collect();
    println!();
    if matches.is_empty() {
        colored_print(Color::Red, "No matches");
    } else {
        println!("Matches:");
        for m in &matches {
            let color = if matches.len() == 1 {
                Color::Green
            } else if m.1 {
                Color::White
            } else {
                Color::DarkGrey
            };
            colored_print(color, &format!("- {}\n", m.0));
        }
    }
}

fn print_start_words() {
    let words = ["slate", "carle", "stare", "roate"];
    println!(
        "No filter defined yet. Good starting words:\n- {}",
        words.join("\n- ")
    );
}

fn colored_print(c: Color, s: &str) {
    _ = execute!(stdout(), SetForegroundColor(c), Print(s), ResetColor);
}

fn process_input(input_mode: InputMode, filter: &mut Filter) -> InputMode {
    let key = read_key();
    if key.modifiers != event::KeyModifiers::NONE {
        println!("Invalid input");
        return input_mode;
    }
    match key.code {
        // user selects to filter on 'must occur' or 'must not occur'
        event::KeyCode::Char('+') | event::KeyCode::Char('-') => {
            let must = key.code == event::KeyCode::Char('+');
            match input_mode {
                InputMode::Positional(x, _) => InputMode::Positional(x, must),
                InputMode::Global(_) => InputMode::Global(must),
            }
        }
        // user selects a position to filter on
        event::KeyCode::Char(ch) if ('1'..='5').contains(&ch) => {
            let pos = ch.to_digit(10).unwrap() as usize - 1;
            let must = match input_mode {
                InputMode::Positional(_, x) => x,
                InputMode::Global(x) => x,
            };
            InputMode::Positional(pos, must)
        }
        // user selects to filter globally
        event::KeyCode::Esc | event::KeyCode::Char('*') => DEFAULT_INPUT_MODE,
        // user selects a character to filter on
        event::KeyCode::Char(ch) if ch.is_ascii_lowercase() => {
            match input_mode {
                InputMode::Positional(x, true) => {
                    // Filter is 'in position x, character must be y'.
                    // Any possibly existing positional filter can be discarded.
                    filter.positional[x] = Some(PositionalFilter::MustBe(ch));
                }
                InputMode::Positional(x, false) => {
                    // Filter is 'in position x, character must not be y'.
                    match filter.positional[x] {
                        None | Some(PositionalFilter::MustBe(_)) => {
                            filter.positional[x] = Some(PositionalFilter::MustNotBe(vec![ch]));
                        }
                        Some(PositionalFilter::MustNotBe(ref mut vec)) => {
                            vec.push(ch);
                            vec.sort();
                        }
                    }
                    // add the character to the 'must occur' list, as the yellow indicator in wordle means character is in the word, but not at position
                    if !filter.must_occur.contains(&ch) {
                        filter.must_occur.push(ch);
                        filter.must_occur.sort();
                    }
                }
                InputMode::Global(true) => {
                    filter.must_occur.push(ch);
                    filter.must_occur.sort();
                }
                InputMode::Global(false) => {
                    filter.must_not_occur.push(ch);
                    filter.must_not_occur.sort();
                }
            }
            input_mode
        }
        // invalid input
        _ => {
            println!("Invalid input");
            input_mode
        }
    }
}

fn read_words_from_file(
    filename: impl AsRef<Path>,
    word_length: usize,
) -> Result<Vec<(String, bool)>> {
    // The file is expected to contain words with a leading + or -.
    // A + indicates a frequent word.
    Ok(read_to_string(filename)?
        .lines()
        .filter(|x| x.len() == word_length + 1)
        .map(|s| (s[1..].to_string(), s.starts_with('+')))
        .collect())
}

pub fn read_key() -> KeyEvent {
    loop {
        let input = event::read().unwrap();
        if let event::Event::Key(key) = input {
            if key.kind == KeyEventKind::Release {
                return key;
            }
        }
    }
}
