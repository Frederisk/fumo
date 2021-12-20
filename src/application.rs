use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub fn create_application() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .args(&[
            Arg::with_name("ascii_distro")
                .long("ascii_distro")
                .short("distro")
                .value_name("DISTRO")
                .long_help(
                    "Which Distro's ascii art to print

NOTE: hakurei_reimu",
                ),
            Arg::with_name("size")
                .long("size")
                .value_name("SIZE_TYPE")
                .long_help(
                    "The size variants of the ascii art

NOTE: auto, small, normal, large",
                )
                .default_value("auto"),
        ])
}
