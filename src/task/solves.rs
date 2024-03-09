use actix_web::web::{Data};
use log::{error};

pub async fn product(tasks: &str) -> u128{

    let res: Result<u128, _> = tasks
        .split_whitespace()
        .map(|s| s.parse::<u128>())
        .product();


    return match res {
        Ok(o) => {
            o
        }
        Err(e) => {
            error!("Error parsing: {}", e);
            0
        }
    }
}