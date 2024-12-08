// アプリケーションの設定を管理するための構造体
#[derive(Debug, Clone)] // Debug: 構造体をデバッグ出力可能にする。Clone: 構造体を複製可能にする。
pub struct Config {
    pub database_url: String, // データベース接続用のURL（例: "postgres://user:password@localhost:5432/mydb"）
    pub jwt_secret: String,  // JWT（JSON Web Token）を生成する際に使用する秘密鍵
    pub jwt_maxage: i64,     // JWTの有効期限（秒単位）
    pub port: u16,           // アプリケーションがリッスンするポート番号
}

// Config構造体に関連するメソッドを定義
impl Config {
    // アプリケーション設定を初期化するためのメソッド
    pub fn init() -> Config {
        // 環境変数 "DATABASE_URL" の値を取得
        // 値が設定されていない場合、プログラムを終了しエラーメッセージを表示
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"); // "DATABASE_URLが設定されていません" というエラーを出力

        // 環境変数 "JWT_SECRET_KEY" の値を取得
        // 値が設定されていない場合、同様にプログラムを終了
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

        // 環境変数 "JWT_MAXAGE" の値を取得
        // 値が設定されていない場合、プログラムを終了
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");

        // Config構造体を作成し、初期化した値を格納
        Config {
            database_url, // 環境変数から取得したデータベースURL
            jwt_secret,   // 環境変数から取得したJWT秘密鍵
            jwt_maxage: jwt_maxage.parse::<i64>().unwrap(), // 環境変数の値を文字列から数値に変換
            port: 8000,   // サーバーのポート番号をデフォルトで8000に設定
        }
    }
}