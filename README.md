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
