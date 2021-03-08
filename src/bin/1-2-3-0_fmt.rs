use std::fmt::{self, Display, Formatter};

struct City {
  name: &'static str,
  // 緯度
  latitude: f32,
  // 経度
  longitude: f32,
}

impl Display for City {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let latitude_c = if self.latitude >= 0.0 { "N" } else { "S" };
    let longitude_c = if self.longitude >= 0.0 { "E" } else { "W" };

    write!(
      f,
      "{}: {:.3}°{} {:.3}°{}",
      self.name,
      self.latitude.abs(),
      latitude_c,
      self.longitude.abs(),
      longitude_c
    )
  }
}

fn main() {
  for city in [
    City {
      name: "ダブリン",
      latitude: 53.347778,
      longitude: -6.259722,
    },
    City {
      name: "オスロ",
      latitude: 59.95,
      longitude: 10.75,
    },
    City {
      name: "バンクーバー",
      latitude: 49.25,
      longitude: -123.1,
    },
  ]
  .iter()
  {
    println!("{}", *city);
  }
}
