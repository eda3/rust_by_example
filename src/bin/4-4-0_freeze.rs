fn main() {
  let mut _mutable_integer = 7i32;
  {
    // _mutable_integer をイミュータブルにシャドーイング
    let _mutable_integer = _mutable_integer;

    // Error! _mutable_integerはこのスコープではフリーズしている
    // _mutable_integer = 50;
    //
    // |     _mutable_integer = 50;
    // |     ^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  }
}
