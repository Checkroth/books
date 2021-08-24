use warp::Filter;
use books::models::Author;
use books::crud::*;


fn show_name_impl(name: String) -> String {
    tracing::info!("Saying hello to {}", name);
    format!("Hello, {}", name)
}


fn get_routes() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    // Get Paths
    let show_name = warp::path!("hello" / String).map(show_name_impl);
    let home = warp::path::end().map(|| "Welcome Home");

    // Post Paths
    let create_author = warp::path!("create" / "author" / i32).map(Author::create);

    // Route combination
    let get_routes = warp::get().and(home.or(show_name));
    let post_routes = warp::post().and(warp::body::json()).and(create_author);
    get_routes.or(get_routes)
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    tracing_subscriber::fmt::init();

    let hello = warp::path!("hello" / String)
        .map(show_name_impl)
        .with(warp::trace::request());
        // .with(warp::trace::named("hello!"));

    let port = 3030;
    println!("serving on {}", port);
    warp::serve(get_routes().with(warp::trace::request()))
        .run(([0, 0, 0, 0], port))
        .await;
    println!("Shutting down...");
}
