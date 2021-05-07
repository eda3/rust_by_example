// 以下では、変数の作成から破棄までのライフタイムを線で示しています
// iは最長のライフタイムを持ち、そのスコープはborrow1およびborrow2
// のスコープを完全に包含します。borrow1とborrow2の存続期間は一切
// 重なりません
fn main() {
  let i = 3; // Lifetime for `i` starts. ────────────────┐
             //                                                     │
  {
    //                                                   │
    let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
                      //                                                ││
    println!("borrow1: {}", borrow1); //              ││
  } // `borrow1 ends. ──────────────────────────────────┘│
    //                                                     │
    //                                                     │
  {
    //                                                   │
    let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
                      //                                                ││
    println!("borrow2: {}", borrow2); //              ││
  } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
} // Lifetime ends. ─────────────────────────────────────┘
