use crate::mailers::auth::AuthMailer;
use crate::models::{_entities::users, users as users_model};
use axum::http::StatusCode;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct RegisterBody {
    name: String,
    email: String,
    password: String,
    password_confirmation: String,
}

#[derive(Debug, Deserialize)]
struct LoginBody {
    name: String,
    password: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    name: String,
    kind: i32,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
    user: UserResponse,
    token: String,
}

#[derive(Serialize)]
struct LogoutResponse {
    message: String,
    user: Option<()>,
}

/// POST /api/user — ユーザー登録
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(body): Json<RegisterBody>,
) -> Result<Response> {
    if body.password != body.password_confirmation {
        return Err(Error::BadRequest(
            "パスワードと確認用パスワードが一致しません".into(),
        ));
    }

    let params = users_model::RegisterParams {
        email: body.email,
        password: body.password,
        name: body.name,
    };

    let user = users::Model::create_with_password(&ctx.db, &params)
        .await
        .map_err(|err| match err {
            ModelError::EntityAlreadyExists => {
                Error::BadRequest("このメールアドレスは既に登録されています".into())
            }
            ModelError::Message(ref msg) if msg == "name already exists" => {
                Error::BadRequest("このユーザー名は既に使われています".into())
            }
            _ => Error::BadRequest(
                "入力内容をご確認ください（メールアドレスの形式などを確認してください）".into(),
            ),
        })?;

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;
    if let Err(err) = AuthMailer::notify_admin_of_registration(&ctx, &user).await {
        tracing::error!(error = ?err, "failed to send admin registration notification");
    }

    let jwt_secret = ctx.config.get_jwt_config()?;
    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .map_err(|_| Error::BadRequest("認証トークンの生成に失敗しました".into()))?;

    let body = format::json(LoginResponse {
        message: "登録しました".into(),
        user: UserResponse {
            id: user.id,
            name: user.name,
            kind: user.kind,
        },
        token,
    })?;
    Ok((StatusCode::CREATED, body).into_response())
}

/// GET /api/user — 現在のログインユーザー取得
#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(UserResponse {
        id: user.id,
        name: user.name,
        kind: user.kind,
    })
}

/// POST /api/session — ログイン
#[debug_handler]
async fn login(State(ctx): State<AppContext>, Json(body): Json<LoginBody>) -> Result<Response> {
    let user = users::Model::find_by_name(&ctx.db, &body.name)
        .await
        .map_err(|_| Error::BadRequest("ユーザー名またはパスワードが違います".into()))?;

    if !user.verify_password(&body.password) {
        return Err(Error::BadRequest(
            "ユーザー名またはパスワードが違います".into(),
        ));
    }

    let jwt_secret = ctx.config.get_jwt_config()?;
    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .map_err(|_| Error::BadRequest("認証トークンの生成に失敗しました".into()))?;

    format::json(LoginResponse {
        message: "ログインしました".into(),
        user: UserResponse {
            id: user.id,
            name: user.name,
            kind: user.kind,
        },
        token,
    })
}

/// DELETE /api/session — ログアウト
#[debug_handler]
async fn logout(_auth: auth::JWT, _state: State<AppContext>) -> Result<Response> {
    format::json(LogoutResponse {
        message: "ログアウトしました".into(),
        user: None,
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/api/user", post(register))
        .add("/api/user", get(current))
        .add("/api/session", post(login))
        .add("/api/session", delete(logout))
}
