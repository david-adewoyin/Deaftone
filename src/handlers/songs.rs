use axum::{
    body::{boxed, Body, BoxBody},
    extract::{Extension, Path, State},
    http::{Request, Response, StatusCode},
};
use sea_orm::DatabaseConnection;
use tower_http::services::fs::ServeFile;

use tower::util::ServiceExt;

use crate::{services, AppState};

pub async fn stream_handler(
    Path(song_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res: Request<Body> = Request::builder().uri("/").body(Body::empty()).unwrap();
    let song: Option<entity::songs::Model> = services::song::get_song(&state.database, song_id)
        .await
        .unwrap();
    match song {
        Some(f) => match ServeFile::new(f.path).oneshot(res).await {
            Ok(res) => Ok(res.map(boxed)),
            Err(err) => Err((
                StatusCode::NOT_FOUND,
                format!("Something went wrong: {}", err),
            )),
        },
        None => Err((StatusCode::NOT_FOUND, format!("Unable to find song"))),
    }
}
