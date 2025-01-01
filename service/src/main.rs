mod domain;

use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hello = warp::get().map(|| "hello, warp!".to_string());
    warp::serve(hello).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
