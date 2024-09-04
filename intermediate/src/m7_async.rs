#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_calls_async_fn() {
        dbg!("Hello...");
    }
}
