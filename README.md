# Markdown parser

My first Rust tutorial, taken from [Jesse Lawson](https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/#how-to-write-to-a-file-in-rust)

Extended to accept fenced code blocks

```
Like this
```

# Usage

```
rustmd file.md
```

Produces `file.html` output. Accepts `H1`s, single-line paragraphs and fenced code blocks.
