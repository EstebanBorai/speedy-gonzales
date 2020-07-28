mod speed_test;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut speed_test = speed_test::SpeedTest::new();

  speed_test.test_connection().await?;
  speed_test.print_results();

  Ok(())
}
