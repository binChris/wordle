# Wordle solver

A small app written in Rust to solve wordle puzzles, like [Wordguessr](https://engaging-data.com/wordguessr-wordle/) or the original of [NYT](https://www.nytimes.com/games/wordle).

There is currently no binary and the app is only tested on Windows. To run the app use:

```sh
cargo run
```

The app suggests some starting words, but you can of course choose your own.

To filter the word list, just type any character. The following filters are supported:
- Character at position x must be c. Example: press `1+a` to filter for words starting with 'a'.
- Character at position x must NOT be c. Example: press `3-x` to exclude all words with an 'x' in the middle.
- Word must contain character. Example: press `esc`, `+y` to only show words containing a 'y'.
- Word must NOT contain character. Example: press `esc`, `-w` to exclude all words containing a 'w'.

The 'must contain' filter can contain the same character multiple times, so you can filter for 'tt' and get 'butts' etc.

Less frequent words in the result list are displayed greyed out.

## How to use the wordle solver

Start the application when starting the puzzle.

For each round, enter one of the words presented. 
- for green characters, add a filter 'in position x char must be c'
- for yellow characters, add a filter 'in position x char must NOT be c'
  - a 'word must contain' filter is automatically added
- for black characters, add a filter 'word must NOT contain'

This will result in a shorter list of words being displayed. Rare words are displayed greyed out and should be chosen after more frequent words. Repeat.

Note: Watch out for black characters that also have a green match - these must not be added to the filter. I might fix this issue at some time.

## What the app does not do

This is a generic solver that contains (a lot) more words that the original [NYT wordle](https://www.nytimes.com/games/wordle) solution list. You can of courst supply your own list of solutions. It also does not remove past solutions from the filtered list.

That said, you still should be able to solve almost any wordle puzzle, even if your active vocabulary does not contain words like ['parer'](https://wordlearchive.com/454).