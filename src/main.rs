use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_markdown_file(filename: &str) {
  print_banner();

  // Use a Path object as the filename
  let filepath = Path::new(filename);

  // Try to open the file
  let file = File::open(&filepath).expect("[ ERROR ]: File not found");

  println!("Parsing {}", filename);

  // Assuming all is well, set up some flags to track where we are in parsing
  // the document
  let mut _ptag: bool = false;
  let mut _htag: bool = false;
  let mut _codetag: bool = false;

  // This vector stores the generated HTML output
  let mut output: Vec<String> = Vec::new();

  // Create a new BufReader to do the reading
  let reader = BufReader::new(file);

  // Loop through each line of the file
  for line in reader.lines() {
    let input_line = line.unwrap();
    let mut output_line = String::new();
    let mut first_char: Vec<char> = input_line.chars().take(1).collect();
    // Depending on the first char we characterise the line as a heading (#),
    // code block (`) or normal text (anything else)
    match first_char.pop() {
      Some('#') => {
        // This line is a header

        // Check if we need to close any tags
        if _ptag {
          _ptag = false;
          output_line.push_str("</p>\n");
        }

        if _htag {
          _htag = false;
          output_line.push_str("</h1>\n");
        }

        if _codetag {
          _codetag = false;
          output_line.push_str("</code>\n>");
        }

        // Create a new header tag
        _htag = true;
        output_line.push_str("\n\n<h1>");
        // Push the rest of the line (minus first 2 chars) to output
        output_line.push_str(&input_line[2..]);
      },
      Some('`') => {
        // We are in a code block

        // Check if we need to close any tags
        if _ptag {
          _ptag = false;
          output_line.push_str("</p>\n");
        }

        if _htag {
          _htag = false;
          output_line.push_str("</h1>\n");
        }

        if _codetag {
          // Close code tag and move on
          _codetag = false;
          output_line.push_str("</code>\n");
        } else {
          // Create a new code tag
          _codetag = true;
          output_line.push_str("<code>");
        }
      },
      _ => {
        // Normal text
        if !_ptag && !_codetag {
          _ptag = true;
          output_line.push_str("<p>");
        }

        output_line.push_str(&input_line);

      }
    }

    // Check if we need to close any tags
    if _ptag {
      _ptag = false;
      output_line.push_str("</p>\n");
    }

    if _htag {
      _htag = false;
      output_line.push_str("</h1>\n");
    }

    // Check that we're not going to push an empty paragraph tag
    if output_line != "<p></p>\n" {
      println!("{}", output_line);
      output.push(output_line);
    }
  }

}

fn print_banner() {
  let pkgname: String = String::from(env!("CARGO_PKG_NAME"));
  let version: String = String::from(env!("CARGO_PKG_VERSION"));
  let desc: String = String::from(env!("CARGO_PKG_DESCRIPTION"));
  println!("{} (v{}): {}", pkgname, version, desc);
}

fn usage() {
  println!("Usage: rustmd <file>");
}

fn main() {
  // Collect command line arguments
  let args: Vec<String> = std::env::args().collect();

  // Check how many args were passed. If anything other than 2, displa
  match args.len() {
    2 => parse_markdown_file(&args[1]),
    _ => usage()
  }
}
