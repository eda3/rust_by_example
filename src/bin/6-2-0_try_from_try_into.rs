// Similar to From and Into, TryFrom and TryInto are generic traits for converting between types.
// FromやIntoと同様に、TryFromやTryIntoは型間の変換を行うための汎用的なcrate

// Unlike From/Into, the TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.
// From/Intoとは異なり、TryFrom/TryInto trateは誤りやすい変換に使用され、そのような場合はResultを返す

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, Pa)]
struct EvenNumber(i32);

fn main() {}
