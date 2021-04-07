use std::marker::PhantomData;

// ジェネリックなタプル構造体。2つ目のパラメータは幽霊型
#[derive(PartialEq)] // 比較演算子(`==`)での比較を可能にする。
struct PhantomTuple<A, B>(A, PhantomData<B>);

// 同様に構造体を定義
#[derive(PartialEq)] // 比較演算子での比較を可能にする。
struct PhantomStruct<A, B> {
  first: A,
  phantom: PhantomData<B>,
}

fn main() {}
