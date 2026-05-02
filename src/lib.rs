mod routes;
mod types;

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use axum::Router;
use axum::response::IntoResponse;
use serde_json::json;

pub use routes::app;
pub use types::*;

#[derive(Debug)]
pub struct AddonState {
    library_path: Option<PathBuf>,
    library: OnceLock<Result<LibraryFile, String>>,
}

impl AddonState {
    pub fn from_env() -> Self {
        Self {
            library_path: std::env::var_os("TYPENX_VIDEO_LIBRARY_FILE").map(PathBuf::from),
            library: OnceLock::new(),
        }
    }

    pub fn with_library_path(path: impl Into<PathBuf>) -> Self {
        Self {
            library_path: Some(path.into()),
            library: OnceLock::new(),
        }
    }

    pub fn default_library() -> Self {
        Self {
            library_path: None,
            library: OnceLock::new(),
        }
    }

    pub fn library(&self) -> Result<&LibraryFile, AddonError> {
        self.library
            .get_or_init(|| {
                read_configured_library(self.library_path.clone()).map_err(|err| err.to_string())
            })
            .as_ref()
            .map_err(|message| AddonError::new(message.clone()))
    }
}

pub type SharedState = Arc<AddonState>;

#[derive(Debug, Clone)]
pub struct AddonError {
    message: String,
}

impl AddonError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for AddonError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AddonError {}

impl IntoResponse for AddonError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(json!({ "message": self.message })),
        )
            .into_response()
    }
}

pub fn manifest() -> AddonManifest {
    AddonManifest {
        id: "typenx-addon-video-library".to_owned(),
        name: "Typenx Video Library".to_owned(),
        version: "0.1.0".to_owned(),
        description: Some("Self-hosted video source distribution addon for Typenx.".to_owned()),
        icon: Some(
            "https://raw.githubusercontent.com/typenx/typenx-addon-video-library/main/icon.png"
                .to_owned(),
        ),
        resources: vec![
            AddonResource::Catalog,
            AddonResource::Search,
            AddonResource::AnimeMeta,
            AddonResource::EpisodeMeta,
            AddonResource::VideoSources,
        ],
        catalogs: vec![CatalogDefinition {
            id: "library".to_owned(),
            name: "Video Library".to_owned(),
            content_type: ContentType::Anime,
            filters: vec![],
        }],
    }
}

pub fn health(state: &AddonState) -> Result<AddonHealth, AddonError> {
    let library = state.library()?;
    Ok(AddonHealth {
        ok: true,
        message: Some(format!("{} shows loaded", library.shows.len())),
    })
}

pub fn catalog(state: &AddonState, request: CatalogRequest) -> Result<CatalogResponse, AddonError> {
    let library = state.library()?;
    let skip = request.skip.unwrap_or(0);
    let limit = request.limit.unwrap_or(24);
    Ok(CatalogResponse {
        items: library
            .shows
            .iter()
            .skip(skip)
            .take(limit)
            .map(to_preview)
            .collect(),
    })
}

pub fn search(state: &AddonState, request: SearchRequest) -> Result<CatalogResponse, AddonError> {
    let library = state.library()?;
    let query = request.query.trim().to_lowercase();
    let limit = request.limit.unwrap_or(24);
    Ok(CatalogResponse {
        items: library
            .shows
            .iter()
            .filter(|show| searchable_text(show).contains(&query))
            .take(limit)
            .map(to_preview)
            .collect(),
    })
}

pub fn anime(state: &AddonState, id: &str) -> Result<AnimeMetadata, AddonError> {
    let show = find_show(state, id)?;
    Ok(to_metadata(show))
}

pub fn videos(
    state: &AddonState,
    request: VideoSourceRequest,
) -> Result<VideoSourceResponse, AddonError> {
    let show = find_show(state, &request.anime_id)?;
    let episode = find_episode(show, &request)?;
    Ok(VideoSourceResponse {
        streams: episode
            .streams
            .iter()
            .map(|stream| VideoStream {
                id: stream.id.clone(),
                title: stream.title.clone(),
                url: stream.url.clone(),
                quality: stream.quality.clone(),
                format: stream.format.clone(),
                audio_language: stream.audio_language.clone(),
                headers: stream.headers.clone().unwrap_or_default(),
            })
            .collect(),
        subtitles: episode.subtitles.clone().unwrap_or_default(),
    })
}

fn read_configured_library(
    path: Option<PathBuf>,
) -> Result<LibraryFile, Box<dyn std::error::Error>> {
    let Some(path) = path else {
        return Ok(default_library());
    };
    let json = std::fs::read_to_string(path)?;
    let value: serde_json::Value = serde_json::from_str(&json)?;
    if !value.get("shows").is_some_and(serde_json::Value::is_array) {
        return Err("TYPENX_VIDEO_LIBRARY_FILE must contain a shows array".into());
    }
    let library: LibraryFile = serde_json::from_value(value)?;
    Ok(library)
}

fn default_library() -> LibraryFile {
    LibraryFile {
        shows: vec![LibraryShow {
            id: "sample-anime".to_owned(),
            title: "Sample Anime".to_owned(),
            synopsis: Some("A sample self-hosted anime entry used to verify Typenx video addon wiring.".to_owned()),
            year: Some(2026),
            content_type: Some(ContentType::Anime),
            genres: Some(vec!["Adventure".to_owned()]),
            episodes: vec![LibraryEpisode {
                id: "sample-anime-1".to_owned(),
                number: 1.0,
                title: Some("Episode 1".to_owned()),
                duration_minutes: Some(10.0),
                streams: vec![VideoStreamInput {
                    id: "sample-720p".to_owned(),
                    title: Some("Sample 720p".to_owned()),
                    url: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4".to_owned(),
                    quality: Some("720p".to_owned()),
                    format: Some("mp4".to_owned()),
                    audio_language: Some("en".to_owned()),
                    headers: Some(vec![]),
                }],
                subtitles: Some(vec![]),
                ..LibraryEpisode::default()
            }],
            ..LibraryShow::default()
        }],
    }
}

fn find_show<'a>(state: &'a AddonState, id: &str) -> Result<&'a LibraryShow, AddonError> {
    state
        .library()?
        .shows
        .iter()
        .find(|show| show.id == id)
        .ok_or_else(|| AddonError::new(format!("Show not found: {id}")))
}

fn find_episode<'a>(
    show: &'a LibraryShow,
    request: &VideoSourceRequest,
) -> Result<&'a LibraryEpisode, AddonError> {
    show.episodes
        .iter()
        .find(|episode| {
            if request
                .episode_id
                .as_ref()
                .is_some_and(|episode_id| episode.id == *episode_id)
            {
                return true;
            }

            request.episode_number.is_some_and(|number| {
                episode.number == number
                    && (request.season_number.is_none()
                        || episode.season_number == request.season_number)
            })
        })
        .ok_or_else(|| {
            let episode = request
                .episode_id
                .clone()
                .or_else(|| request.episode_number.map(|number| number.to_string()))
                .unwrap_or_else(|| "unknown".to_owned());
            AddonError::new(format!("Episode not found for {}: {episode}", show.id))
        })
}

fn to_preview(show: &LibraryShow) -> AnimePreview {
    AnimePreview {
        id: show.id.clone(),
        title: show.title.clone(),
        poster: show.poster.clone(),
        banner: show.banner.clone(),
        synopsis: show.synopsis.clone().or_else(|| show.description.clone()),
        score: show.score,
        year: show.year.or(show.season_year),
        content_type: show.content_type.clone().unwrap_or(ContentType::Anime),
        genres: show.genres.clone().unwrap_or_default(),
    }
}

fn to_metadata(show: &LibraryShow) -> AnimeMetadata {
    AnimeMetadata {
        id: show.id.clone(),
        title: show.title.clone(),
        original_title: show.original_title.clone(),
        alternative_titles: show.alternative_titles.clone().unwrap_or_default(),
        synopsis: show.synopsis.clone(),
        description: show.description.clone().or_else(|| show.synopsis.clone()),
        poster: show.poster.clone(),
        banner: show.banner.clone(),
        year: show.year.or(show.season_year),
        season: show.season.clone(),
        season_year: show.season_year.or(show.year),
        status: show.status.clone(),
        content_type: show.content_type.clone().unwrap_or(ContentType::Anime),
        source: show.source.clone(),
        duration_minutes: None,
        episode_count: Some(show.episodes.len()),
        score: show.score,
        rank: None,
        popularity: None,
        rating: show.rating.clone(),
        genres: show.genres.clone().unwrap_or_default(),
        tags: show.tags.clone().unwrap_or_default(),
        authors: show.authors.clone().unwrap_or_default(),
        studios: show.studios.clone().unwrap_or_default(),
        staff: vec![],
        country_of_origin: show.country_of_origin.clone(),
        start_date: None,
        end_date: None,
        site_url: show.site_url.clone(),
        trailer_url: show.trailer_url.clone(),
        external_links: vec![],
        episodes: show
            .episodes
            .iter()
            .map(|episode| to_episode_metadata(&show.id, episode))
            .collect(),
        updated_at: Some(chrono::Utc::now().to_rfc3339()),
    }
}

fn to_episode_metadata(anime_id: &str, episode: &LibraryEpisode) -> EpisodeMetadata {
    EpisodeMetadata {
        id: episode.id.clone(),
        anime_id: anime_id.to_owned(),
        season_number: episode.season_number,
        number: episode.number,
        title: episode.title.clone(),
        synopsis: episode.synopsis.clone(),
        thumbnail: episode.thumbnail.clone(),
        duration_minutes: episode.duration_minutes,
        source: Some("video_library".to_owned()),
        aired_at: episode.aired_at.clone(),
    }
}

fn searchable_text(show: &LibraryShow) -> String {
    let mut values = vec![show.title.clone()];
    values.extend(show.original_title.clone());
    values.extend(show.alternative_titles.clone().unwrap_or_default());
    values.extend(show.genres.clone().unwrap_or_default());
    values.join(" ").to_lowercase()
}

pub fn router_from_env() -> Router {
    app(Arc::new(AddonState::from_env()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_defaults_to_first_twenty_four_items() {
        let state = AddonState::default_library();
        let response = catalog(&state, CatalogRequest::default()).unwrap();
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].id, "sample-anime");
        assert_eq!(response.items[0].content_type, ContentType::Anime);
    }

    #[test]
    fn search_matches_title_alternative_title_and_genre() {
        let state = AddonState::default_library();
        let response = search(
            &state,
            SearchRequest {
                query: "adventure".to_owned(),
                limit: Some(10),
                ..SearchRequest::default()
            },
        )
        .unwrap();
        assert_eq!(response.items.len(), 1);
    }

    #[test]
    fn anime_metadata_preserves_episode_shape() {
        let state = AddonState::default_library();
        let response = anime(&state, "sample-anime").unwrap();
        assert_eq!(response.episode_count, Some(1));
        assert_eq!(
            response.episodes[0].source.as_deref(),
            Some("video_library")
        );
        assert!(response.updated_at.is_some());
    }

    #[test]
    fn video_sources_can_find_episode_by_number() {
        let state = AddonState::default_library();
        let response = videos(
            &state,
            VideoSourceRequest {
                anime_id: "sample-anime".to_owned(),
                episode_number: Some(1.0),
                ..VideoSourceRequest::default()
            },
        )
        .unwrap();
        assert_eq!(response.streams[0].id, "sample-720p");
        assert!(response.streams[0].headers.is_empty());
        assert!(response.subtitles.is_empty());
    }

    #[test]
    fn unknown_show_matches_typescript_error_message() {
        let state = AddonState::default_library();
        let error = anime(&state, "missing").unwrap_err();
        assert_eq!(error.message(), "Show not found: missing");
    }

    #[test]
    fn invalid_library_matches_typescript_error_message() {
        let path = std::env::temp_dir().join(format!(
            "typenx-video-library-invalid-{}.json",
            std::process::id()
        ));
        std::fs::write(&path, "{}").unwrap();

        let state = AddonState::with_library_path(&path);
        let error = health(&state).unwrap_err();

        std::fs::remove_file(path).unwrap();
        assert_eq!(
            error.message(),
            "TYPENX_VIDEO_LIBRARY_FILE must contain a shows array"
        );
    }
}
