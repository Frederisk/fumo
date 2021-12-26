use std::collections::HashMap;

pub fn ascii_normal<'a, 'b>() -> HashMap<&'a str, &'b str> {
    [
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
        (
            "kirisame_marisa",
            r#"                  ___-_
              __--.....---__
           _--..............---__
         _/......................\-_
        /...........................\-_
      _-...........___......._===_.....\
     /......___----...---__./#####\.....
    /...__--...............-\#####|.__..
   /___-..._____----____.....<====|/##\_
_---...__---_-----____- ---__.......==-_
...___-***|  ____     ____ -|---__......
_-- |||***|------    -------|****|-__...
    \||***||0|  |    |0|  | |****|   \..
--__--\***\\-  _/    \-  _/ /***|/_  /..
      ||***\ --        --  |***_/  ---__
      \*\****\__ -__-     _|**/
      \-=--___\__    ___-- |*/
       |###||    -==-    |_|/
      _/--/  \ o  == o  /   \_
     / \*//__/ o     o  \-__-\\
     \__\|/ ==___     __===\   \
       //   /####-----###\  -_-/=
      =/   |##############|      \\
    //      \=#########==/        \\
   \\====___  \____>===  __==_  _=//
      /  ---=__      _ ==--  -==-
     /       --======--/      |
    |       /         /       /
     --____-          \__  __/
                         --"#,
        ),
    ]
    .into()
}

#[cfg(test)]
mod fumo_ascii_tests {
    use crate::processing::DistroSize;

    use super::*;

    #[test]
    fn test_ascii_normal() {
        ascii_normal()
            .values()
            .map(ToOwned::to_owned)
            .map(str::lines)
            .for_each(|lines| {
                lines.map(str::len).for_each(|length: usize| {
                    assert!(length <= DistroSize::Normal as usize);
                });
            });
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
