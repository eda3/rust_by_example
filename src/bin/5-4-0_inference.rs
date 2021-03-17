fn type_of<T>(_: T) -> String {
  let a = std::any::type_name::<T>();
  a.to_string()
}
fn main() {
  // アノテーションのおかげで、コンパイラはelemがu8型であることがわかる
  let elem = 5u8;
  println!("{}", type_of(elem));
  // -> u8

  // 空のベクトル(可変長の配列)を生成
  let mut vec = Vec::new();

  // この時点でvecの型がVec<u8>だと型推論される
  vec.push(elem);

  println!("{:?}", vec);
  // -> [5]
  println!("{}", type_of(vec));
  // -> alloc::vec::Vec<u8>
}
