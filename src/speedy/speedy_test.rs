use crate::speedy::SpeedyTestError;
use std::time::{Instant};
use reqwest::Client;

const FILE_BYES: u32 = 5230592;

pub struct SpeedyTest {
  pub speed_bps: f32,
  pub speed_kbps: f32,
  pub speed_mbps: f32,
  pub request_duration: u128,
  http_client: Client,
}

impl SpeedyTest {
  pub fn new() -> Self {
    let http_client = Client::builder()
      .build()
      .unwrap();
  
    SpeedyTest {
      speed_bps: 0.0,
      speed_kbps: 0.0,
      speed_mbps: 0.0,
      request_duration: 0,
      http_client
    }
  }

  pub async fn test_connection(&mut self) -> Result<(), SpeedyTestError> {
    let now = Instant::now();
    let bits_loaded = FILE_BYES * 8;

    self.http_client.get("https://s3-us-west-2.amazonaws.com/uw-s3-cdn/wp-content/uploads/sites/6/2017/02/04143141/20130821_P1000415.jpg")
      .send()
      .await?;

    self.request_duration = now.elapsed().as_millis();   
    self.speed_bps = bits_loaded as f32 /  (self.request_duration as f32);
    self.speed_kbps = self.speed_bps / 1024 as f32;
    self.speed_mbps = self.speed_kbps / 1024 as f32;

    Ok(())
  }

  pub fn print_results(&self) {
    println!("Speedy Results:\n");

    println!("Bytes Downloaded: {} bytes\n", FILE_BYES);

    println!("Download:");
    println!("{} bps", self.speed_bps);
    println!("{} kbps", self.speed_kbps);
    println!("{} Mbps", self.speed_mbps);
    println!("{} ms", self.request_duration);
  }
}
