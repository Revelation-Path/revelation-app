use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response
};
use uuid::Uuid;

pub struct AuthLayer;

impl AuthLayer {
    /// Extract user UUID from X-User-ID header or cookie
    pub async fn extract_user(mut request: Request, next: Next) -> Result<Response, StatusCode> {
        let user_id = Self::get_user_id(&request);

        if let Some(user_id) = user_id {
            request.extensions_mut().insert(UserId(user_id));
        }

        Ok(next.run(request).await)
    }

    fn get_user_id(request: &Request) -> Option<Uuid> {
        // Try X-User-ID header first
        if let Some(header) = request.headers().get("X-User-ID")
            && let Ok(s) = header.to_str()
            && let Ok(uuid) = Uuid::parse_str(s)
        {
            return Some(uuid);
        }

        // Try cookie
        if let Some(cookie) = request.headers().get(header::COOKIE)
            && let Ok(s) = cookie.to_str()
        {
            for part in s.split(';') {
                let part = part.trim();
                if let Some(value) = part.strip_prefix("user_id=")
                    && let Ok(uuid) = Uuid::parse_str(value)
                {
                    return Some(uuid);
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct UserId(pub Uuid);
