// `A`という具象型
struct A;

// `Single`という型を定義する際に`A`を使用しているが、その最初の使用よりも
// 先に`<A>`がないため、また、`A`自身も具象型であるため、`Single`は具象型となる。
struct Single(A);

// ここでは`<T>`が一番初めの`T`の使用よりも先に来ている。よって`SingleGen`はジェネリック型
// となる。なぜならば型パラメータ`T`がジェネリックだからである。`T`はどんな型にもなりえるため、
// 上で定義した`A`を受け取ることもできる。
struct SingleGen<T>(T);

fn main() {}
