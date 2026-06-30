use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    errors::PlanrError,
    features::{auth::service::validate_token, users::UserId},
    state::AppState,
};

#[derive(Clone)]
pub struct AuthUser {
    pub user_id: UserId,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(PlanrError::Unauthorized)?;

    let claims = validate_token(token, &state.jwt_secret).map_err(|_| PlanrError::Unauthorized)?;

    req.extensions_mut().insert(AuthUser {
        user_id: claims.sub.into(),
    });

    Ok::<Response, PlanrError>(next.run(req).await)
}
