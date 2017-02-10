extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn update(stdin:&mut std::io::Stdin, stdout:&mut std::io::Stdout) -> () {
  for c in stdin.keys() {
     match c {
      Ok(key) => {
        match key {
            Key::Char(c) => {
               draw(stdout);
            },
            _ => {}
        }
      },
      Err(_) => {}
     }
  }
}

fn draw(stdout:&mut std::io::Stdout) -> () {
  write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();  
    for red in 0..32 {
        let red = red * 8;
        for green in 0..64 {
            let green = green * 4;
            write!(stdout, "{} ", termion::color::Bg(termion::color::Rgb(red, green, 25))).unwrap();
        }
        write!(stdout, "\n\r").unwrap();
    }
writeln!(stdout, "{}", termion::style::Reset).unwrap();
  stdout.flush().unwrap();
}

fn main() -> () {
  let mut stdin: std::io::Stdin = stdin();
  let mut stdout: std::io::Stdout = stdout();  
  update(&mut stdin, &mut stdout);
}
