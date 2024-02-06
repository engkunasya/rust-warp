// use crate::{
//     security::{do_auth, UserCtx},
//     with_db_pool, DbPool,
// };
use serde_json::{json, Value};
use std::sync::Arc;

use warp::{reply::Json, Filter};

// pub fn todos_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     // LISI TODO
//     warp::path("todos").and(warp::get()).map(|| {
//         let todo = "will get todo!";
//         warp::reply::json(&json!({ "todo": todo }))
//     })
// }

// ----SYNCRHONOUS MAPPING
// .map(): This method is used to transform the input value of a filter or route handler. It takes a closure or function literal and applies it to the input value, producing a new value. The closure can perform any arbitrary transformation or computation on the input value and return a result that implements the warp::Reply trait.

// ----ASYNC
// .and_then(): This method is used to handle the output value of a filter or route handler that returns a Result or a Future. It takes a closure or function literal that receives the output value, and it can perform additional processing or error handling on the output. The closure should return a result or future that implements the warp::Reply trait.

//-----------------CONTINUE ESOK

pub fn todos_filter(
    db_pool: Arc<DbPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todos_base = warp::path("todos");
    // LIST todos
    let list = todos_base
        .and(warp::get())
        .and(warp::path::end())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and_then(todo_list);

    let get = todos_base
        .and(warp::get())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::path::param()) // e.g., /todos/123
        .and_then(todo_get);

    let create = todos_base
        .and(warp::post())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::body::json())
        .and_then(todo_create);

    list.or(get).or(create)
}

async fn todo_list(_user_ctx: UserCtx, _db_pool: Arc<DbPool>) -> Result<Json, warp::Rejection> {
    // TODO - get from DB
    let todos = json!([
        {"id": 1, "title": "todo 1"},
        {"id": 2, "title": "todo 2"}
    ]);

    let todos = warp::reply::json(&todos);

    Ok(todos)
}

async fn todo_get(
    _user_ctx: UserCtx,
    _db_pool: Arc<DbPool>,
    id: i64,
) -> Result<Json, warp::Rejection> {
    // TODO - get from DB
    let todo = json!(
        {"id": id, "user_id": _user_ctx.user_id, "title": format!("todo {}", id)}
    );

    // serde-json to warp-reply
    let todo = warp::reply::json(&todo);

    Ok(todo)
}

async fn todo_create(
    _user_ctx: UserCtx,
    _db_pool: Arc<DbPool>,
    data: Value,
) -> Result<Json, warp::Rejection> {
    // TODO - write to DB
    let todo = data;

    let todo = warp::reply::json(&todo);

    Ok(todo)
}
