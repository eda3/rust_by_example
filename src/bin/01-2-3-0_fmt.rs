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

#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
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
  // -> 以下のとおり出力
  // ダブリン: 53.348°N 6.260°W
  // オスロ: 59.950°N 10.750°E
  // バンクーバー: 49.250°N 123.100°W

  for color in [
    Color {
      red: 128,
      green: 255,
      blue: 90,
    },
    Color {
      red: 0,
      green: 3,
      blue: 254,
    },
    Color {
      red: 0,
      green: 0,
      blue: 0,
    },
  ]
  .iter()
  {
    println!("{:?}", *color);
  }
  // -> 以下のとおり出力
  // Color { red: 128, green: 255, blue: 90 }
  // Color { red: 0, green: 3, blue: 254 }
  // Color { red: 0, green: 0, blue: 0 }
}
