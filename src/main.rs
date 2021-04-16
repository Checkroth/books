use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    tracing_subscriber::fmt::init();

    let hello = warp::path!("hello" / String)
        .map(|name| {
            tracing::info!("Saying hello to");
            format!("Hello, {}!", name)
        })
        .with(warp::trace::request());
        // .with(warp::trace::named("hello!"));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030))
        .await;
    println!("Shutting down...");
}
