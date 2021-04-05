struct Years(i64);
struct Days(i64);

impl Years {
  pub fn to_days(&self) -> Days {
    Days(self.0 * 365)
  }
}

impl Days {
  pub fn to_years(&self) -> Years {
    Years(self.0 / 365)
  }
}

fn old_enough(age: &Years) -> bool {
  age.0 >= 18
}

fn main() {}
