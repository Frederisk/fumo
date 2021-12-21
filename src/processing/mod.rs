use std::collections::HashMap;

use crate::Term;
use clap::{ArgMatches, Error, ErrorKind};

mod ascii_map;

const DISTRO_SMALL_MAX_SIZE: u16 = 20;
const DISTRO_NORMAL_MAX_SIZE: u16 = 40;
// const DISTRO_LARGE_MAX_SIZE: u16 = 80;

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

fn print_fumo(terminal: Term, distro: &str, size: DistroSize) -> () {
    let fumo_map = match size {
        DistroSize::Large => HashMap::new(), // TODO: implement large size
        DistroSize::Normal => ascii_map::ascii_normal(),
        DistroSize::Small => HashMap::new(), // TODO: implement small size
    };
    if let Some(fumo) = fumo_map.get(distro) {
        fn fail_to_print_and_exit(err: std::io::Error) -> () {
            Error::with_description(
                format!("failed to write fumo: {}", err).as_str(),
                ErrorKind::Io,
            )
            .exit();
        }

        terminal
            .write_line(fumo)
            .unwrap_or_else(fail_to_print_and_exit);
        terminal.flush().unwrap_or_else(fail_to_print_and_exit);
    } else {
        Error::with_description("invalid `ascii_distro` value", ErrorKind::InvalidValue).exit();
    }
}
