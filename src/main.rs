mod todo_rest;
use todo_rest::todos_filter;

use serde_json::json;
use warp::Filter;

#[tokio::main]

/* HTML RESPONSE */
// async fn main() {
//     let hello_world = warp::path::end().map(|| "Hello from root!");

//     println!("start web server!");
//     warp::serve(hello_world).run(([127, 0, 0, 1], 8080)).await;
// }

/* JSON RESPONSE */
async fn main() {
    let root = warp::path::end().and(warp::get()).map(|| {
        let message = "Hello from root!";
        warp::reply::json(&json!({ "message": message }))
    });

    let hi = warp::path("hi").and(warp::get()).map(|| {
        let extension = "Hello from hi extension";
        warp::reply::json(&json!({ "hi": extension }))
    });
    let apis = hi.or(todos_filter());

    let routes = root.or(apis);

    println!("start web server below!");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

// FILTER FUNCTIONALITY: MEANS CONTROLLER TO PERFORM GET/POST METHOD AT SPECIFIED PATH

// AT BAREBONE: ENDPOINT AND PATH ONLY, BUT GET/POST/DEL ARENT RELEVANT = SAME THING
