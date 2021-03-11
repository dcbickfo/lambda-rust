// This example requires the following input to succeed:
// { "command": "do something" }

use lambda_runtime::{handler_fn, Context};
use serde_json::Value;

/// A shorthand for `Box<dyn std::error::Error + Send + Sync + 'static>`
/// type required by aws-lambda-rust-runtime.
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: Value, _: Context) -> Result<Value, Error> {
    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(event)
}