use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddonResource {
    Catalog,
    Search,
    AnimeMeta,
    EpisodeMeta,
    VideoSources,
    MangaPages,
    Recommendations,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Anime,
    Manga,
    Manhwa,
    Manhua,
    LightNovel,
    Movie,
    Ova,
    Ona,
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub resources: Vec<AddonResource>,
    pub catalogs: Vec<CatalogDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogDefinition {
    pub id: String,
    pub name: String,
    pub content_type: ContentType,
    pub filters: Vec<CatalogFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogFilter {
    pub id: String,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CatalogRequest {
    pub addon_id: Option<String>,
    pub catalog_id: Option<String>,
    pub content_type: Option<ContentType>,
    pub skip: Option<usize>,
    pub limit: Option<usize>,
    pub query: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SearchRequest {
    pub addon_id: Option<String>,
    #[serde(default)]
    pub query: String,
    pub content_type: Option<ContentType>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct VideoSourceRequest {
    pub addon_id: Option<String>,
    #[serde(default)]
    pub anime_id: String,
    pub anime_title: Option<String>,
    pub episode_id: Option<String>,
    pub episode_title: Option<String>,
    pub episode_number: Option<f64>,
    pub season_number: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogResponse {
    pub items: Vec<AnimePreview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimePreview {
    pub id: String,
    pub title: String,
    pub poster: Option<String>,
    pub banner: Option<String>,
    pub synopsis: Option<String>,
    pub score: Option<f64>,
    pub year: Option<i64>,
    pub content_type: ContentType,
    pub genres: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeMetadata {
    pub id: String,
    pub title: String,
    pub original_title: Option<String>,
    pub alternative_titles: Vec<String>,
    pub synopsis: Option<String>,
    pub description: Option<String>,
    pub poster: Option<String>,
    pub banner: Option<String>,
    pub year: Option<i64>,
    pub season: Option<String>,
    pub season_year: Option<i64>,
    pub status: Option<String>,
    pub content_type: ContentType,
    pub source: Option<String>,
    pub duration_minutes: Option<f64>,
    pub episode_count: Option<usize>,
    pub score: Option<f64>,
    pub rank: Option<i64>,
    pub popularity: Option<i64>,
    pub rating: Option<String>,
    pub genres: Vec<String>,
    pub tags: Vec<String>,
    pub authors: Vec<String>,
    pub studios: Vec<String>,
    pub staff: Vec<StaffCredit>,
    pub country_of_origin: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub site_url: Option<String>,
    pub trailer_url: Option<String>,
    pub external_links: Vec<ExternalLink>,
    pub episodes: Vec<EpisodeMetadata>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffCredit {
    pub name: String,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalLink {
    pub site: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeMetadata {
    pub id: String,
    pub anime_id: String,
    pub season_number: Option<f64>,
    pub number: f64,
    pub title: Option<String>,
    pub synopsis: Option<String>,
    pub thumbnail: Option<String>,
    pub duration_minutes: Option<f64>,
    pub source: Option<String>,
    pub aired_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonHealth {
    pub ok: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoSourceResponse {
    pub streams: Vec<VideoStream>,
    pub subtitles: Vec<VideoSubtitle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoStream {
    pub id: String,
    pub title: Option<String>,
    pub url: String,
    pub quality: Option<String>,
    pub format: Option<String>,
    pub audio_language: Option<String>,
    pub headers: Vec<VideoHeader>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VideoSubtitle {
    pub id: String,
    pub label: String,
    pub language: Option<String>,
    pub url: String,
    pub format: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LibraryFile {
    pub shows: Vec<LibraryShow>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibraryShow {
    pub id: String,
    pub title: String,
    pub original_title: Option<String>,
    pub alternative_titles: Option<Vec<String>>,
    pub poster: Option<String>,
    pub banner: Option<String>,
    pub synopsis: Option<String>,
    pub description: Option<String>,
    pub year: Option<i64>,
    pub season: Option<String>,
    pub season_year: Option<i64>,
    pub status: Option<String>,
    pub content_type: Option<ContentType>,
    pub source: Option<String>,
    pub score: Option<f64>,
    pub rating: Option<String>,
    pub genres: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub studios: Option<Vec<String>>,
    pub authors: Option<Vec<String>>,
    pub country_of_origin: Option<String>,
    pub site_url: Option<String>,
    pub trailer_url: Option<String>,
    pub episodes: Vec<LibraryEpisode>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibraryEpisode {
    pub id: String,
    pub number: f64,
    pub season_number: Option<f64>,
    pub title: Option<String>,
    pub synopsis: Option<String>,
    pub thumbnail: Option<String>,
    pub duration_minutes: Option<f64>,
    pub aired_at: Option<String>,
    pub streams: Vec<VideoStreamInput>,
    pub subtitles: Option<Vec<VideoSubtitle>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoStreamInput {
    pub id: String,
    pub title: Option<String>,
    pub url: String,
    pub quality: Option<String>,
    pub format: Option<String>,
    pub audio_language: Option<String>,
    pub headers: Option<Vec<VideoHeader>>,
}
