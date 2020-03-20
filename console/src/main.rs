use std::thread;
use std::time::Duration;
use console::Term;

fn main() {
    let term = Term::stdout();
    term.write_line("Hello World!");
    thread::sleep(Duration::from_millis(2000));
    print!("\x1B[2J");
}
