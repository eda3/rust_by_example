struct A; // 具象型A
struct S(A); // 具象型Ｓ
struct SGen<T>(T); // ジェネリック型SGen

// 以下の関数は全て変数の所有権をとった後すぐにスコープを抜けて
// 変数をメモリ上から開放する。

// `S`という型の引数`_s`をとる`reg_fn`という関数を定義
// `<T>`がないのでジェネリック関数ではない
fn reg_fn(_s: S) {}

// `gen_spec_t`という関数を定義。これは`A`という型を与えられた`Sgen<T>`
// という型の引数`_s`を取る。関数名の直後に`<A>`という型パラメータでAが
// ジェネリックであることを明示していないので、この関数はAをジェネリック型
// としては取らない
fn gen_spec_t(_s: SGen<A>) {}

// `gen_spec_i32`という関数を定義。
// これは明示的な型パラメータとして`i32`を与えられた`Sgen<i32>`型の引数`_s`をとる
// この関数もジェネリックではない
fn gen_spec_i32(_s: SGen<i32>) {}

// `generic`という関数を定義。`SGen<T>`という型の引数`_s`を取る。`<T>`が`SGen<T>`に
// 先行しているため、これはTに対してジェネリックな関数
fn generic<T>(_s: SGen<T>) {}

fn main() {
  // ジェネリックでない関数を使用する
  reg_fn(S(A)); // 具象型

  gen_spec_t(SGen(A)); // 型パラメータ`A`を暗黙のうちに受け取る

  gen_spec_i32(SGen(6)); // 型パラメータ`i32`を暗黙のうちに受け取る

  // 型パラメータ `char` を `generic()` に明示的に指定
  generic::<char>(SGen('a'));

  // 型パラメータ`char`を暗黙的に`generic()`に渡す
  generic(SGen('c'));
}
