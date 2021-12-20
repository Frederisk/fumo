use crate::Term;
use clap::{ArgMatches, Error, ErrorKind};

const DISTRO_SMALL_MAX_SIZE: u16 = 20;
const DISTRO_NORMAL_MAX_SIZE: u16 = 40;
const DISTRO_LARGE_MAX_SIZE: u16 = 80;

enum DistroSize {
    Small,
    Large,
    Normal,
}

pub fn process_matches(matches: ArgMatches, terminal: Term) -> () {
    if let Some(ascii_distro) = matches.value_of("ascii_distro") {
        let size = parse_size(matches.value_of("size").unwrap(), &terminal);
        print_fumo(terminal, ascii_distro, size);
    } else {
        Error::with_description(
            "`ascii_distro` is undefined",
            ErrorKind::MissingArgumentOrSubcommand,
        )
        .exit();
    }
}

/// Parse size string to `DistroSize`
fn parse_size(size: &str, terminal: &Term) -> DistroSize {
    match size {
        "small" => DistroSize::Small,
        "large" => DistroSize::Large,
        "normal" => DistroSize::Normal,
        "auto" => {
            // get terminal size and adjust it to a specific size.
            if let Some((_, c)) = terminal.size_checked() {
                if c <= DISTRO_SMALL_MAX_SIZE {
                    DistroSize::Small
                } else if c <= DISTRO_NORMAL_MAX_SIZE {
                    DistroSize::Normal
                } else {
                    DistroSize::Large
                }
            // if terminal size is not available, use normal size.
            } else {
                DistroSize::Normal
            }
        }
        // exit if size is not defined.
        _ => {
            Error::with_description("invalid `size` value", ErrorKind::InvalidValue).exit();
        }
    }
}

fn print_fumo(terminal: Term, distro: &str, size: DistroSize) -> () {}

fn match_name(input: &str, name: &str) -> bool {
    // name.split('_'); TODO: implement
    if name == input {
        true
    } else {
        false
    }
}
