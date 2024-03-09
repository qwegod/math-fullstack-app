use log::error;

pub async fn product(tasks: &str) -> u128 {
    let res: Result<u128, _> = tasks
        .split_whitespace()
        .map(|s| s.parse::<u128>())
        .product();

    return res.unwrap_or_else(|e| {
        error!("Error parsing: {}", e);
        0
    });
}
