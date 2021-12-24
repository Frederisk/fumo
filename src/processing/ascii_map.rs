use std::collections::HashMap;

pub fn ascii_normal<'a, 'b>() -> HashMap<&'a str, &'b str> {
    HashMap::<&str, &str>::from([
        (
            "hakurei_reimu",
            r#"                              __<====>
 /\                       _<==++++++/+\
/++>==>__             _<=/+++++++++/+++|
|++\+++++====>__     /+++++++++++++\++/
|+++\+++++++++++<===>+++++++++++++++\+|
|++/++++++++++==/.<==\+++++++++++++/++|
 \++\+++=====/.........\<========++\++/
 |+==-//-..................\_    \===/
 |/   |......................\
     /........................\
     |....../\......../-_.....|
     |...._/--\_.....|---\-_..|
     /.._/ _____\_..|______ \.|
    |../   |0|  | --/|0|  | |\\
    \|||_| \-   /    \   / /-|\|
     \||#\  \__/      --- /|#|
       /\=|\__    __    _- |-|
      _/..|_/ --_____---\_ /.|_-\
    /- \..| -_--/__\---\- |...|/|
    \|  |/   _/+|  |+++|   |_/ ||
    |\      /+++|   \+++\_     /|
     \|/----++++\__-/+++++-____/
     /+++++++++++++++++++++++--__
      \+++++++++++++++++++++++++_\
     __<===_+++++++++++++/=====>
    /     | <=+++++__===>   \
    |     /   <==-/   /      |
     \-_-/            \      /
                       \-__-/"#,
        ),
        ("kirisame_marisa", ""),
    ])
}

#[cfg(test)]
mod fumo_ascii_tests {
    use crate::processing::DistroSize;

    use super::*;

    #[test]
    fn test_ascii_normal() {
        ascii_normal().values().for_each(|fumo| {
            fumo.lines()
                .for_each(|line| assert!(line.len() <= DistroSize::Normal as usize));
        })
    }

    #[test]
    fn test_ascii_small() {
        // TODO: implement small size test.
    }

    #[test]
    fn test_ascii_large() {
        // TODO: implement large size test.
    }
}
