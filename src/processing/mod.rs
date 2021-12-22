use crate::Term;
use clap::{ArgMatches, Error, ErrorKind};
use std::collections::HashMap;

mod ascii_map;

const DISTRO_SMALL_MAX_SIZE: usize = 20;
const DISTRO_NORMAL_MAX_SIZE: usize = 40;
// const DISTRO_LARGE_MAX_SIZE: usize = 80;

/// Available fumo sizes.
enum DistroSize {
    Small,
    Large,
    Normal,
}

pub fn process_matches(matches: ArgMatches, terminal: Term) {
    // if the corresponding fumo is found, print it.
    let ascii_distro = matches.value_of("ascii_distro").unwrap_or_else(|| ""); // TODO: returns the appropriate value according to the environment.
    let size = parse_size(matches.value_of("size").unwrap(), &terminal);
    let fumo_lines = get_fumo_lines(ascii_distro, size);
    let system_info_lines = get_system_info_lines();
}

/// Parse size string to `DistroSize`.
fn parse_size(size: &str, terminal: &Term) -> DistroSize {
    match size {
        "small" => DistroSize::Small,
        "large" => DistroSize::Large,
        "normal" => DistroSize::Normal,
        "auto" => {
            // get terminal size and adjust it to a specific size.
            if let Some((_, c)) = terminal.size_checked() {
                if c <= DISTRO_SMALL_MAX_SIZE as u16 {
                    DistroSize::Small
                } else if c <= DISTRO_NORMAL_MAX_SIZE as u16 {
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
            clap::Error::with_description("Invalid `size` value", ErrorKind::InvalidValue).exit();
        }
    }
}

fn get_fumo_lines<'a>(distro: &str, size: DistroSize) -> Vec<&'a str> {
    // select a map.by size.
    let fumo_map = match size {
        DistroSize::Large => HashMap::new(), // TODO: implement large size
        DistroSize::Normal => ascii_map::ascii_normal(),
        DistroSize::Small => HashMap::new(), // TODO: implement small size
    };
    // try to get a fumo from map and print it.

    fumo_map
        .get(distro)
        .unwrap_or_else(|| {
            Error::with_description("Invalid `ascii_distro` value", ErrorKind::InvalidValue).exit()
        })
        .lines()
        .collect()

    // if let Some(fumo) = fumo_map.get(distro) {
    //     fumo.lines().collect()
    // } else {
    //     Error::with_description("Invalid `ascii_distro` value", ErrorKind::InvalidValue).exit();
    // }
}

fn get_system_info_lines<'a>() -> Vec<String> {
    use whoami::*;

    let mut info_lines = Vec::<String>::new();

    let user_info = format!("{}@{}", username(), hostname());
    let user_info_len = user_info.len();

    info_lines.push(user_info); // username@hostname
    info_lines.push(format!("{:-<width$}", "", width = user_info_len)); // -----------------
    info_lines.push(format!("OS: {}", os_info::get())); // OS: System Name

    info_lines
}

fn foo() {
    let info = vec![
        whoami::desktop_env().to_string(),             // Windows
        whoami::devicename(),                          // Desktop-Frederisk
        whoami::distro(),                              // Windows 10.0.19044 (Workstation)
        whoami::hostname(),                            // DESKTOP-FREDERI
        whoami::lang().collect::<Vec<_>>().join(", "), // zh-TW, en-US
        whoami::platform().to_string(),                // Windows
        whoami::realname(),                            // Frederisk
        whoami::username(),                            // Frederisk
    ];
}
