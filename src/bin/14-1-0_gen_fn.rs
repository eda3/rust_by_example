struct A; // 具象型A
struct S(A); // 具象型Ｓ
struct SGen<T>(T); // ジェネリック型SGen

fn main() {}
