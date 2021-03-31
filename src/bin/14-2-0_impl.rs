struct S; // 具象型`S`

struct GenericVal<T>(T); // ジェネリック型`GenericVal`

// 型パラメータを指定したうえで、GenericValにメソッドを実装
impl GenericVal<f32> {} // `f32`の場合のメソッド

impl GenericVal<S> {} // 上で定義した`S`への実装

// ジェネリック型のまま扱うには`<T>`が先に来る必要がある。
impl<T> GenericVal<T> {}

fn main() {}
