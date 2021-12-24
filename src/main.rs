use clap::{Error, ErrorKind};
use console::Term;

mod application;
mod processing;

fn main() {
    let terminal = Term::buffered_stderr();
    if !terminal.is_term() {
        Error::with_description("Not a terminal", ErrorKind::Io).exit();
    }

    let application: clap::App = application::create_application();
    let matches = application.get_matches();

    processing::process_matches(matches, terminal);
}
