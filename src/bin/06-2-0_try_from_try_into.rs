// Similar to From and Into, TryFrom and TryInto are generic traits for converting between types.
// FromやIntoと同様に、TryFromやTryIntoは型間の変換を行うための汎用的なcrate

// Unlike From/Into, the TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.
// From/Intoとは異なり、TryFrom/TryInto traitは誤りやすい変換に使用され、そのような場合はResultを返す

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
  type Error = ();

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    if value % 2 == 2 {
      Ok(EvenNumber(value))
    } else {
      Err(())
    }
  }
}

fn main() {
  // Try From
  assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
  assert_eq!(EvenNumber::try_from(5), Err(()));

  // TryInto
  let result: Result<EvenNumber, ()> = 8i32.try_into();
  assert_eq!(result, Ok(EvenNumber(8)));
  let result: Result<EvenNumber, ()> = 5i32.try_into();
  assert_eq!(result, Err(()));
}
