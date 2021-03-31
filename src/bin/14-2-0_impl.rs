struct S; // 具象型`S`

struct GenericVal<T>(T); // ジェネリック型`GenericVal`

// 型パラメータを指定したうえで、GenericValにメソッドを実装
impl GenericVal<f32> {} // `f32`の場合のメソッド

impl GenericVal<S> {} // 上で定義した`S`への実装

// ジェネリック型のまま扱うには`<T>`が先に来る必要がある。
impl<T> GenericVal<T> {}

struct Val {
  val: f64,
}

// Valに対してimpl
impl Val {
  fn value(&self) -> &f64 {
    &self.val
  }
}

struct GenVal<T> {
  gen_val: T,
}

// ジェネリック型`T`の場合のメソッドをGenValに対して実装
impl<T> GenVal<T> {
  fn value(&self) -> &T {
    &self.gen_val
  }
}

fn main() {
  let x = Val { val: 3.0 };
  let y = GenVal { gen_val: 3i32 };

  println!("x:{}, y:{}", x.value(), y.value());
  // x:3, y:3
}
