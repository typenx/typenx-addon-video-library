use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;

use crate::{
    AddonError, AddonState, CatalogRequest, SearchRequest, SharedState, VideoSourceRequest, anime,
    catalog, health, manifest, search, videos,
};

pub fn app(state: Arc<AddonState>) -> Router {
    Router::new()
        .route("/health", get(health_route))
        .route("/health/", get(health_route))
        .route("/manifest", get(manifest_route))
        .route("/manifest/", get(manifest_route))
        .route("/catalog", post(catalog_route))
        .route("/catalog/", post(catalog_route))
        .route("/search", post(search_route))
        .route("/search/", post(search_route))
        .route("/anime/{id}", get(anime_route))
        .route("/anime/{id}/", get(anime_route))
        .route("/videos", post(videos_route))
        .route("/videos/", post(videos_route))
        .fallback(not_found)
        .with_state(state)
}

async fn health_route(State(state): State<SharedState>) -> Result<impl IntoResponse, AddonError> {
    Ok(Json(health(&state)?))
}

async fn manifest_route() -> impl IntoResponse {
    Json(manifest())
}

async fn catalog_route(
    State(state): State<SharedState>,
    Json(request): Json<CatalogRequest>,
) -> Result<impl IntoResponse, AddonError> {
    Ok(Json(catalog(&state, request)?))
}

async fn search_route(
    State(state): State<SharedState>,
    Json(request): Json<SearchRequest>,
) -> Result<impl IntoResponse, AddonError> {
    Ok(Json(search(&state, request)?))
}

async fn anime_route(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AddonError> {
    Ok(Json(anime(&state, &id)?))
}

async fn videos_route(
    State(state): State<SharedState>,
    Json(request): Json<VideoSourceRequest>,
) -> Result<impl IntoResponse, AddonError> {
    Ok(Json(videos(&state, request)?))
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": "Not found" })),
    )
}
