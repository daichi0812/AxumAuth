use chrono::prelude::*; // 日付や時刻の操作に必要な型やトレイト（例: DateTime, Utc）
use serde::{Deserialize, Serialize}; // 構造体や列挙型をJSONなどに変換（Serialize）、またはその逆に変換（Deserialize）

// ユーザーの役割を表す列挙型（例: 管理者(Admin) または 一般ユーザー(User)）
#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)] // トレイトの自動実装
#[sqlx(type_name = "user_role", rename_all = "lowercase")] // データベースでこの型をENUMとしてマッピングし、小文字で保存
pub enum UserRole {
    Admin,
    User, 
}

// UserRole に追加のメソッドを定義
impl UserRole {
    // UserRole を文字列として返すメソッド
    pub fn to_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin", // Admin を "admin" という文字列に変換
            UserRole::User => "user",  // User を "user" という文字列に変換
        }
    }
}

// ユーザー情報を表す構造体
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, sqlx::Type, Clone)] // トレイトの自動実装
pub struct User {
    pub id: uuid::Uuid, 
    pub name: String, 
    pub email: String, 
    pub password: String, 
    pub role: UserRole, 
    pub verified: bool, 
    pub verification_token: Option<String>, 
    pub token_expires_at: Option<DateTime<Utc>>, 
    #[serde(rename = "createdAt")] // JSONシリアライズ時のキー名を "createdAt" に変更
    pub created_at: Option<DateTime<Utc>>, 
    #[serde(rename = "updatedAt")] // JSONシリアライズ時のキー名を "updatedAt" に変更
    pub updated_at: Option<DateTime<Utc>>, 
}