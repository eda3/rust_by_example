// NanoSecond を u64の別名として使用する
type NanoSecond = u64;
type Inch = u64;

// キャメルケースじゃない時の警告を抑える
#[allow(non_camel_case_types)]
type u64_t = u64;
// ^^^^^ help: convert the identifier to upper camel case: `U64T`
// #[allow(non_camel_case_types)]がないとき、↑の警告が起こる

fn main() {
  let nanoseconds: NanoSecond = 5 as u64_t;
  let inches: Inch = 2 as u64_t;

  // 型のエイリアスは元の型をより型安全にしれくれる **わけではない ** ことに注意しましょう
  println!(
    "{} nanoseconds + {} inches = {} unit?",
    nanoseconds,
    inches,
    nanoseconds + inches
  );
  // -> 5 nanoseconds + 2 inches = 7 unit?
}
