use screen_buffer::{Buffer, console::Style};

fn main() {
    let mut buf = Buffer::new();
    let term = console::Term::stdout();
    term.clear_screen().unwrap();
    buf.puts(0, 0, "Hello World\n".to_string(), Style::new());
    buf.flush();
}
