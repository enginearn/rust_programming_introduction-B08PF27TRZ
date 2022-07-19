use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

/*
エラーをまとめるenumを定義
actix_web::ResponseErrorとして使うために
deriveマクロでDebugを付与している必要がある
*/
#[derive(Debug, Error)]
enum MyError {
    #[error("Failed to render HTML!")]
    AskamaError(#[from] askama::Error),

    #[error("Failed to get connection...")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Failed SQL execution...")]
    SQLiteError(#[from] rusqlite::Error),
}

// actix_web::ResponseErrorをMyErrorに実装
impl ResponseError for MyError {}

// MyErrorはactix_web::ResponseErrorを実装しているので
// indexの戻り値にMyErrorを使うことができる
#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id, text })
    })?;

    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }

    let html = IndexTemplate{entries};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))

    /*
    HttpResponse::Ok()はステータスコード 200を持つHttpResponseBuilderという構造体を返す。
    HtpResponseBuilderのbody()という関数にレスポンスボディを渡すと、HttpResponseが返ってくる。
    戻り値の型がResultなのでOkで包む
    */
    // Ok(HttpResponse::Ok().body(response_body))
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("Failed to initialize the connection pool...");
    let conn = pool
        .get()
        .expect("Failed to get the connection from the pool");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    )
    .expect("Failed to create a table `todo`.");
    // ここでコネクションループを渡す
    HttpServer::new(move || App::new().service(index).app_data(pool.clone()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}