// `i32`. The reference to `i32` must outlive `Borrowed`.
// `i32`への参照をメンバに持つ`Borrowed`型。
// 参照は`Borrowed`自体よりも長生きでなくてはならない。
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// 同様に、ここでも参照は構造体よりも長生きでなくてはならない。
#[derive(Debug)]
struct NamedBorrowed<'a> {
  x: &'a i32,
  y: &'a i32,
}

// i32、あるいはi32への参照のいずれかとなる列挙型
enum Either<'a> {
  Num(i32),
  Ref(&'a i32),
}

fn main() {}
