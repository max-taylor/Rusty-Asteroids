use crossterm::style::Color;

use crate::api::display::{map_from_str, Map};

pub const ZERO: &str = "000000
00  00
00  00
00  00
000000";

pub const ONE: &str = "1111
  11
  11
  11
111111";

pub const TWO: &str = "222222
     2
222222
2
222222";

pub const THREE: &str = "333333
    33
333333
    33
333333";

pub const FOUR: &str = "44  44
44  44
444444
    44
    44";

pub const FIVE: &str = "555555
55
555555
    55
555555";

pub const SIX: &str = "666666
66
666666
66  66
666666";

pub const SEVEN: &str = "777777
    77
    77
    77
    77";

pub const EIGHT: &str = "888888
88  88
888888
88  88
888888";

pub const NINE: &str = "999999
99  99
999999
    99
999999";

pub const NUMBER_VECTOR: [&'static str; 10] =
    [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

pub const X: &str = "xx xx
  x
xx xx";
