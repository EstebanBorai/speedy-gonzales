use crate::speed_test::SpeedTestError;
use std::collections::HashMap;

pub struct SpeedTest {
  pub speed_bps: f32,
  pub speed_kbps: f32,
  pub speed_mbps: f32,
}

impl SpeedTest {
  pub fn new() -> Self {
    SpeedTest {
      speed_bps: 0.0,
      speed_kbps: 0.0,
      speed_mbps: 0.0,
    }
  }

  pub async fn test_connection(&mut self) -> Result<(), SpeedTestError> {
    let resp = reqwest::get("https://httpbin.org/ip")
      .await?
      .json::<HashMap<String, String>>()
      .await?;

    self.speed_bps = 1.5;
    self.speed_kbps = 2.5;
    self.speed_mbps = 5.5;

    Ok(())
  }

  pub fn print_results(&self) {
    println!("Connection Speed: ");
    println!("{}", format!("{} bps", self.speed_bps));
    println!("{}", format!("{} kbps", self.speed_kbps));
    println!("{}", format!("{} Mbps", self.speed_mbps));
  }
}
