mod speedy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut speed_test = speedy::SpeedyTest::new();

  speed_test.test_connection().await?;
  speed_test.print_results();

  Ok(())
}
