struct Years(i64);
struct Days(i64);

impl Years {
  pub fn to_days(&self) -> Days {
    Days(self.0 * 365)
  }
}
fn main() {}
