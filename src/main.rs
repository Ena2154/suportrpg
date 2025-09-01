// main.rs

// android_activityクレートから、AndroidApp構造体とMain宏をインポートします
use android_activity::{AndroidApp, Main};

// この関数がアプリのメインの処理となります
fn main_func(app: AndroidApp) {
    // 今はまだ何もしませんが、今後ここに描画処理などを書いていきます
    loop {
        // イベントを処理します（例：画面タッチ、閉じるボタンなど）
        let mut events = app.events();
        if let Some(event) = events.next() {
            // イベントに応じた処理をここに書きます
            // 例: log::info!("Event: {:?}", event);
        }
    }
}

// Main宏が、Android OSから呼び出されるエントリーポイントを自動で生成してくれます
#[Main(main_func)]
struct App;
