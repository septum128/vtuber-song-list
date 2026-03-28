pub mod channels;
pub mod song_diffs;
pub mod song_items;
pub mod videos;

use crate::models::_entities::users;
use loco_rs::prelude::*;

const USER_KIND_ADMIN: i32 = 10;

/// Validates JWT and ensures the user is an admin.
/// Returns the user model on success, or Unauthorized error.
///
/// # Errors
/// Returns `Error::Unauthorized` if the user is not an admin.
pub async fn require_admin(auth: &auth::JWT, ctx: &AppContext) -> Result<users::Model> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if user.kind != USER_KIND_ADMIN {
        return Err(Error::Unauthorized("管理者権限が必要です".into()));
    }
    Ok(user)
}
