import { readFile } from 'node:fs/promises'
import {
  createTypenxAddon,
  serveTypenxAddon,
  type AddonManifest,
  type AnimeMetadata,
  type AnimePreview,
  type CatalogResponse,
  type ContentType,
  type EpisodeMetadata,
  type VideoSourceRequest,
  type VideoSourceResponse,
  type VideoStream,
  type VideoSubtitle,
} from '@typenx/addon-ts-sdk'

type LibraryFile = {
  shows: LibraryShow[]
}

type LibraryShow = {
  id: string
  title: string
  original_title?: string | null
  alternative_titles?: string[]
  poster?: string | null
  banner?: string | null
  synopsis?: string | null
  description?: string | null
  year?: number | null
  season?: string | null
  season_year?: number | null
  status?: string | null
  content_type?: ContentType
  source?: string | null
  score?: number | null
  rating?: string | null
  genres?: string[]
  tags?: string[]
  studios?: string[]
  authors?: string[]
  country_of_origin?: string | null
  site_url?: string | null
  trailer_url?: string | null
  episodes: LibraryEpisode[]
}

type LibraryEpisode = {
  id: string
  number: number
  season_number?: number | null
  title?: string | null
  synopsis?: string | null
  thumbnail?: string | null
  duration_minutes?: number | null
  aired_at?: string | null
  streams: VideoStream[]
  subtitles?: VideoSubtitle[]
}

const manifest: AddonManifest = {
  id: 'typenx-addon-video-library',
  name: 'Typenx Video Library',
  version: '0.1.0',
  description: 'Self-hosted video source distribution addon for Typenx.',
  icon: 'https://raw.githubusercontent.com/typenx/typenx-addon-video-library/main/icon.png',
  resources: ['catalog', 'search', 'anime_meta', 'episode_meta', 'video_sources'],
  catalogs: [
    {
      id: 'library',
      name: 'Video Library',
      content_type: 'anime',
      filters: [],
    },
  ],
}

let libraryPromise: Promise<LibraryFile> | null = null

const addon = createTypenxAddon({
  manifest,
  handlers: {
    health: async () => {
      const library = await loadLibrary()
      return {
        ok: true,
        message: `${library.shows.length} shows loaded`,
      }
    },
    catalog: async (request): Promise<CatalogResponse> => {
      const library = await loadLibrary()
      const skip = request.skip ?? 0
      const limit = request.limit ?? 24
      return {
        items: library.shows.slice(skip, skip + limit).map(toPreview),
      }
    },
    search: async (request): Promise<CatalogResponse> => {
      const query = request.query.trim().toLowerCase()
      const limit = request.limit ?? 24
      const library = await loadLibrary()
      return {
        items: library.shows
          .filter((show) => searchableText(show).includes(query))
          .slice(0, limit)
          .map(toPreview),
      }
    },
    anime: async (id): Promise<AnimeMetadata> => {
      const show = await findShow(id)
      return toMetadata(show)
    },
    videos: async (request): Promise<VideoSourceResponse> => {
      const show = await findShow(request.anime_id)
      const episode = findEpisode(show, request)
      return {
        streams: episode.streams.map((stream) => ({
          ...stream,
          headers: stream.headers ?? [],
        })),
        subtitles: episode.subtitles ?? [],
      }
    },
  },
})

serveTypenxAddon(addon, {
  port: Number(process.env.PORT ?? 8790),
})

async function loadLibrary(): Promise<LibraryFile> {
  if (!libraryPromise) {
    libraryPromise = readConfiguredLibrary()
  }
  return libraryPromise
}

async function readConfiguredLibrary(): Promise<LibraryFile> {
  const path = process.env.TYPENX_VIDEO_LIBRARY_FILE
  if (!path) return defaultLibrary()

  const json = JSON.parse(await readFile(path, 'utf8')) as LibraryFile
  if (!Array.isArray(json.shows)) {
    throw new Error('TYPENX_VIDEO_LIBRARY_FILE must contain a shows array')
  }
  return json
}

function defaultLibrary(): LibraryFile {
  return {
    shows: [
      {
        id: 'sample-anime',
        title: 'Sample Anime',
        synopsis: 'A sample self-hosted anime entry used to verify Typenx video addon wiring.',
        year: 2026,
        content_type: 'anime',
        genres: ['Adventure'],
        episodes: [
          {
            id: 'sample-anime-1',
            number: 1,
            title: 'Episode 1',
            duration_minutes: 10,
            streams: [
              {
                id: 'sample-720p',
                title: 'Sample 720p',
                url: 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4',
                quality: '720p',
                format: 'mp4',
                audio_language: 'en',
                headers: [],
              },
            ],
            subtitles: [],
          },
        ],
      },
    ],
  }
}

async function findShow(id: string) {
  const library = await loadLibrary()
  const show = library.shows.find((item) => item.id === id)
  if (!show) throw new Error(`Show not found: ${id}`)
  return show
}

function findEpisode(show: LibraryShow, request: VideoSourceRequest) {
  const episode = show.episodes.find((item) => {
    if (request.episode_id && item.id === request.episode_id) return true
    if (
      request.episode_number != null &&
      item.number === request.episode_number &&
      (request.season_number == null || item.season_number === request.season_number)
    ) {
      return true
    }
    return false
  })

  if (!episode) {
    throw new Error(
      `Episode not found for ${show.id}: ${request.episode_id ?? request.episode_number ?? 'unknown'}`,
    )
  }
  return episode
}

function toPreview(show: LibraryShow): AnimePreview {
  return {
    id: show.id,
    title: show.title,
    poster: show.poster ?? null,
    banner: show.banner ?? null,
    synopsis: show.synopsis ?? show.description ?? null,
    score: show.score ?? null,
    year: show.year ?? show.season_year ?? null,
    content_type: show.content_type ?? 'anime',
    genres: show.genres ?? [],
  }
}

function toMetadata(show: LibraryShow): AnimeMetadata {
  return {
    id: show.id,
    title: show.title,
    original_title: show.original_title ?? null,
    alternative_titles: show.alternative_titles ?? [],
    synopsis: show.synopsis ?? null,
    description: show.description ?? show.synopsis ?? null,
    poster: show.poster ?? null,
    banner: show.banner ?? null,
    year: show.year ?? show.season_year ?? null,
    season: show.season ?? null,
    season_year: show.season_year ?? show.year ?? null,
    status: show.status ?? null,
    content_type: show.content_type ?? 'anime',
    source: show.source ?? null,
    duration_minutes: null,
    episode_count: show.episodes.length,
    score: show.score ?? null,
    rank: null,
    popularity: null,
    rating: show.rating ?? null,
    genres: show.genres ?? [],
    tags: show.tags ?? [],
    authors: show.authors ?? [],
    studios: show.studios ?? [],
    staff: [],
    country_of_origin: show.country_of_origin ?? null,
    start_date: null,
    end_date: null,
    site_url: show.site_url ?? null,
    trailer_url: show.trailer_url ?? null,
    external_links: [],
    episodes: show.episodes.map((episode) => toEpisodeMetadata(show.id, episode)),
    updated_at: new Date().toISOString(),
  }
}

function toEpisodeMetadata(animeId: string, episode: LibraryEpisode): EpisodeMetadata {
  return {
    id: episode.id,
    anime_id: animeId,
    season_number: episode.season_number ?? null,
    number: episode.number,
    title: episode.title ?? null,
    synopsis: episode.synopsis ?? null,
    thumbnail: episode.thumbnail ?? null,
    duration_minutes: episode.duration_minutes ?? null,
    source: 'video_library',
    aired_at: episode.aired_at ?? null,
  }
}

function searchableText(show: LibraryShow) {
  return [
    show.title,
    show.original_title,
    ...(show.alternative_titles ?? []),
    ...(show.genres ?? []),
  ]
    .filter(Boolean)
    .join(' ')
    .toLowerCase()
}
