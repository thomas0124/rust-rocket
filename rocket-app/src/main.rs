#[macro_use]
extern crate rocket;

use rocket::State;
use std::sync::Mutex;

// Task構造体を作成
#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
}

// アプリケーションの状態を管理するための構造体
struct AppState {
    tasks: Mutex<Vec<Task>>,
}

// ルートエンドポイント
#[get("/")]
fn index(state: &State<AppState>) -> String {
    let tasks = state.tasks.lock().unwrap();
    let mut result = String::from("<h1>TODO List</h1><ul>");

    for task in tasks.iter() {
        result.push_str(&format!("<li>{}</li>", task.description));
    }

    result.push_str("</ul>");
    result
}

fn main() {
    // アプリケーションの状態を初期化
    let app_state = AppState {
        tasks: Mutex::new(vec![
            Task {
                id: 1,
                description: String::from("Learn Rocket"),
            },
            Task {
                id: 2,
                description: String::from("Build a TODO app"),
            },
        ]),
    };

    // Rocketアプリケーションを起動
    rocket::build()
        .mount("/", routes![index])
        .manage(app_state)
        .launch();
}
