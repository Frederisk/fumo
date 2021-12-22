use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub fn create_application<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .args(&[
            Arg::with_name("ascii_distro")
                .long("ascii_distro")
                .short("distro")
                .takes_value(true)
                .value_name("DISTRO")
                .help("Which Distro's ascii art to print")
                .long_help(
                    "Which Distro's ascii art to print

NOTE: hakurei_reimu, kirisame_marisa",
                ),
            Arg::with_name("size")
                .long("size")
                .short("size")
                .takes_value(true)
                .value_name("SIZE_TYPE")
                .help("The size variants of the ascii art")
                .long_help(
                    "The size variants of the ascii art

NOTE: auto, small, normal, large",
                )
                .default_value("auto"),
            Arg::with_name("disable_info")
                .long("disable_info")
                .takes_value(false)
                .help("Disable printing of system info"),
        ])
}
