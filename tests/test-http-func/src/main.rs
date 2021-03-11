use lambda_http::{
    handler,
    lambda_runtime::{self, Context},
    IntoResponse, Request,
};
use serde_json::json;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(json!({"foo":"bar"}))
}