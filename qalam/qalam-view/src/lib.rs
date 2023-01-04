pub mod tools;
pub mod utils;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BetterResponse {
    module: &'static str,
    message: &'static str,
    content: &'static str,
}
