fn main() {
  let an_integer = 1u32;
  let a_boolean = true;
  let unit = ();

  // an_integerくぉcopied_integerへとコピー
  let copied_integer = an_integer;
  println!("an_integer: {:?}", copied_integer);
  println!("a_boolean: {:?}", a_boolean);
  println!("unit: {:?}", unit);
  // -> an_integer: 1
  // -> a_boolean: true
  // -> unit: ()
}
