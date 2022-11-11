use tokio::time::{sleep, Duration};

pub async fn check_login () {
    sleep(Duration::from_millis(100)).await;
}