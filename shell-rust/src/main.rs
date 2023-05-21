mod command;
mod srceen;
use srceen::Srceen;
fn main() {
    let mut screen = Srceen {};
    loop {
        screen.start();
    }
}
