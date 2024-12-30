use chrono::{DateTime, Utc};
use core::str;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{User, UserRole};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
    // 名前は1文字以上必要
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    // メールアドレスは1文字以上、かつ形式が正しくないとエラー
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,

    // パスワードは最低6文字必要
    #[validate(
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,

    // パスワード確認用。1文字以上かつパスワードと一致しないとエラー
    #[validate(
        length(min = 1, message = "Confirm Password is required"),
        must_match(other = "password", message = "passwords do not match")
    )]
    // JSON上のフィールド名は"passwordConfirm"になる
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    // ログイン用のメールアドレス検証（必須・形式チェック）
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,

    // ログイン用のパスワード検証（6文字以上必要）
    #[validate(
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    // pageは1以上の数値が必須
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    // limitは1以上50以下の数値が必須
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    // 取得したユーザー情報をフィルタリングして返すDTO
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    // JSON上のフィールド名はcreatedAtになる
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    // JSON上のフィールド名はupdatedAtになる
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    // 単一ユーザーオブジェクトをFilterUserDtoに変換するヘルパーメソッド
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            verified: user.verified,
            role: user.role.to_str().to_string(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    // 複数のユーザーをまとめてFilterUserDtoのベクターに変換するヘルパーメソッド
    pub fn filter_users(user: &[User]) -> Vec<FilterUserDto> {
        user.iter().map(FilterUserDto::filter_user).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    // 単一のユーザー情報をまとめるDTO
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    // レスポンスステータスとユーザーデータをまとめるDTO
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    // ユーザー一覧のレスポンスDTO
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    // ログイン時のレスポンスDTO (tokenを付与)
    pub status: String, 
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    // 汎用的なレスポンスメッセージDTO
    pub status: &'static str,
    pub message: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    // ユーザー名更新用DTO、1文字以上必須
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RoleUpdateDto {
    // ユーザーロール更新用DTO、カスタムバリデーション関数を使用
    #[validate(custom(function = "validate_user_role"))]
    pub role: UserRole,
}

// ユーザーロールがAdminかUserであることを検証する関数
fn validate_user_role(role: &UserRole) -> Result<(), validator::ValidationError> {
    match role {
        UserRole::Admin | UserRole::User => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_role")),
    }
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdateDto {
    // 新しいパスワードは6文字以上
    #[validate(
        length(min = 6, message = "new password must be at least 6 characters" )
    )]
    pub new_password: String,

    // 新しいパスワードの確認も6文字以上かつnew_passwordと一致必須
    #[validate(
        length(min = 6, message = "new password confirm must be at least 6 characters"),
        must_match(other = "new_password", message = "new passwords do not match")
    )]
    pub new_password_confirm: String,

    // 古いパスワードも6文字以上必須
    #[validate(
        length(min = 6, message = "Old password must be at least 6 characters")
    )]
    pub old_password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct VerifyEmailQueryDto {
    // メール認証用のトークン必須 (1文字以上)
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct ForgotPasswordRequestDto {
    // パスワード忘れた時にメールを送るDTO。メール必須かつ形式チェック
    #[validate(length(min = 1, message = "Email is required"), email(message = "Email is invalid"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ResetPasswordRequestDto {
    // パスワードリセット用トークン必須
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,

    // 新しいパスワードは6文字以上
    #[validate(
        length(min = 6, message = "New password must be at least 6 characters")
    )]
    pub new_password: String,

    // 新しいパスワード確認は6文字以上かつnew_passwordと一致必須
    #[validate(
        length(min = 6, message = "New password confirm must be at least 6 characters")
    )]
    pub new_password_confirm: String,
}