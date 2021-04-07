use std::marker::PhantomData;

// ジェネリックなタプル構造体。2つ目のパラメータは幽霊型
#[derive(PartialEq)] // 比較演算子(`==`)での比較を可能にする。
struct PhantomTuple<A, B>(A, PhantomData<B>);

fn main() {}
