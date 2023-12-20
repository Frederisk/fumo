use clap::{ArgMatches, Error, ErrorKind};
use console::Term;
use os_info::Type;
use std::borrow::Cow;

mod ascii_map;

/// Available fumo sizes.
#[derive(Clone, Copy)]
enum DistroSize {
    Small = 20,  // const DISTRO_SMALL_MAX_SIZE: usize = 20;
    Normal = 40, // const DISTRO_NORMAL_MAX_SIZE: usize = 40;
    // Large = 80,  // const DISTRO_LARGE_MAX_SIZE: usize = 80;
}

pub fn process_matches(matches: ArgMatches, terminal: Term) {
    // if the corresponding fumo is found, print it.
    let ascii_distro = matches
        .value_of("ascii_distro")
        .unwrap_or_else(get_default_fumo_name);
    let is_disable_info = matches.is_present("disable_info");
    let terminal_width = if let Some((_, column)) = terminal.size_checked() {
        Some(column)
    } else {
        None
    };
    let size = parse_size(
        matches.value_of("size").unwrap(),
        &terminal_width,
        &is_disable_info,
    );
    let fumo = get_fumo(ascii_distro, &size);
    let fumo_lines = fumo.lines().map(Cow::Borrowed).collect::<Vec<Cow<str>>>();
    let lines = if is_disable_info {
        fumo_lines
    } else {
        let info_lines = get_system_info_lines();
        let line_num = std::cmp::max(fumo_lines.len(), info_lines.len()); // TODO: calculates the real line number.
        let mut lines = Vec::with_capacity(line_num);
        for i in 0..line_num {
            lines.push(Cow::Owned(format!(
                "{:<width$}  {}",
                fumo_lines.get(i).unwrap_or(&"".into()),
                info_lines.get(i).unwrap_or(&"".into()),
                width = size as usize // `size` copied here
            )));
        }
        lines
    };
    lines
        // .iter()
        // .map(|line| -> Cow<Cow<str>> {
        //     // substring the line by black magic.
        //     if let Some(width) = terminal_width {
        //         if line.len() > width as usize {
        //             let line: &str = line.borrow();
        //             Cow::Owned(Cow::Owned(
        //                 line.chars().take(width as usize).collect::<String>(),
        //             ))
        //         } else {
        //             Cow::Borrowed(line)
        //         }
        //     } else {
        //         Cow::Borrowed(line)
        //     }
        // })
        // .for_each(|line| {
        //     // i don't know how is it works, but it actually works.
        //     terminal
        //         .write_line({
        //             let line: &Cow<str> = line.borrow();
        //             line.borrow()
        //         })
        //         .unwrap()
        // });
        .into_iter()
        .map(|line| -> Cow<str> {
            if let Some(width) = terminal_width {
                if line.len() > width as usize {
                    return line
                        .as_ref()
                        .chars()
                        .take(width as usize)
                        .collect::<String>()
                        .into();
                }
            }
            line
        })
        // .map(|line| -> String {
        //     line.as_ref()
        //         .chars()
        //         .take(terminal_width.unwrap_or(u16::MAX) as usize)
        //         .collect()
        // })
        .for_each(|line| terminal.write_line(line.as_ref()).unwrap());
    terminal.flush().unwrap();
}

fn get_default_fumo_name<'a>() -> &'a str {
    let os_type = os_info::get().os_type();
    match os_type {
        Type::Alpine => todo!(),
        Type::Amazon => todo!(),
        Type::Android => todo!(),
        Type::Arch => todo!(),
        Type::CentOS => todo!(),
        Type::Debian => "saigyouji_yuyuko",
        Type::DragonFly => todo!(),
        Type::Emscripten => todo!(),
        Type::EndeavourOS => todo!(),
        Type::Fedora => todo!(),
        Type::FreeBSD => todo!(),
        Type::Linux => todo!(),
        Type::Macos => todo!(),
        Type::Manjaro => todo!(),
        Type::Mint => todo!(),
        Type::NetBSD => todo!(),
        Type::NixOS => todo!(),
        Type::openSUSE => todo!(),
        Type::OracleLinux => todo!(),
        Type::Pop => todo!(),
        Type::Raspbian => todo!(),
        Type::Redhat => todo!(),
        Type::RedHatEnterprise => todo!(),
        Type::Redox => todo!(),
        Type::Solus => todo!(),
        Type::SUSE => todo!(),
        Type::Ubuntu => todo!(),
        Type::Unknown => todo!(),
        Type::Windows => "cirno",
        _ => todo!(),
    }
}

// fn combine_lines<'a>(
//     fumo: &'a str,
//     info_lines: Option<Vec<String>>,
//     size: &'a DistroSize,
// ) -> Vec<Cow<'a, str>> {
//     if let Some(info_lines) = info_lines {
//         let fumo_lines = fumo.lines().collect::<Vec<&str>>();
//         let line_num = std::cmp::max(fumo_lines.len(), info_lines.len());
//         let mut lines = Vec::<Cow<str>>::new();
//         for i in 0..line_num {
//             lines.push(Cow::Owned(format!(
//                 "{: <width$}  {}",
//                 fumo_lines.get(i).unwrap_or(&""),
//                 info_lines.get(i).unwrap_or(&String::from("")),
//                 width = *size as usize // `size` copied here
//             )));
//         }
//         lines
//     } else {
//         fumo.lines().map(Cow::Borrowed).collect()
//     }
// }

/// Parse size string to `DistroSize`.
fn parse_size(size: &str, width: &Option<u16>, is_disable_info: &bool) -> DistroSize {
    match size {
        "small" => DistroSize::Small,
        // "large" => DistroSize::Large,
        "normal" => DistroSize::Normal,
        "auto" => {
            // get terminal size and adjust it to a specific size.
            if let Some(width) = *width {
                let column_num = if *is_disable_info {
                    width
                } else {
                    width.saturating_sub(42) // 40 for system info, 2 for the last two spaces.
                };
                if column_num <= DistroSize::Small as u16 {
                    DistroSize::Small
                } else
                // if column_num <= DistroSize::Normal as u16
                {
                    DistroSize::Normal
                }
                // else {
                //     // DistroSize::Large
                // }
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

fn get_fumo<'a>(distro: &str, size: &DistroSize) -> &'a str {
    // select a map.by size.
    let fumo_map = match size {
        // DistroSize::Large => ascii_map::ascii_normal(), // TODO: implement large size
        DistroSize::Normal => ascii_map::ascii_normal(),
        DistroSize::Small => ascii_map::ascii_normal(), // TODO: implement small size
    };
    // try to get a fumo from map and print it.
    fumo_map.get(distro).unwrap_or_else(|| {
        Error::with_description("Invalid `ascii_distro` value", ErrorKind::InvalidValue).exit()
    })
    // if let Some(fumo) = fumo_map.get(distro) {
    //     fumo.lines().collect()
    // } else {
    //     Error::with_description("Invalid `ascii_distro` value", ErrorKind::InvalidValue).exit();
    // }
}

fn get_system_info_lines() -> Vec<String> {
    // TODO: enhance the system info.
    use whoami::*;
    let mut info_lines = Vec::<String>::new();

    let user_info = format!("{}@{}", username(), hostname());
    let user_info_len = user_info.len();
    // blank line
    info_lines.push(String::new());
    // username@HOST_NAM
    info_lines.push(user_info);
    // -----------------
    // info_lines.push(format!("{:-<width$}", "", width = user_info_len));
    info_lines.push("-".repeat(user_info_len));
    // OS: System Name
    info_lines.push(format!("OS: {}", os_info::get()));
    // Device Name: Device-Name
    info_lines.push(format!("DeviceName: {}", devicename()));
    // Real Name: Real_name
    info_lines.push(format!("RealName: {}", realname()));
    // Languages: Language Name, Language Name, ...
    info_lines.push(format!(
        "Languages: {}",
        lang().collect::<Vec<_>>().join(", ")
    ));
    // whoami::desktop_env().to_string(), // Windows
    // whoami::distro(),                  // Windows 10.0.19044 (Workstation)
    // whoami::platform().to_string(),    // Windows
    info_lines
}
