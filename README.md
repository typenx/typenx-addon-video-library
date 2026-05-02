# Typenx Video Library Addon

Official Typenx addon for self-hosted video source distribution.

This addon is intentionally simple: it reads a local JSON library and exposes Typenx addon routes for catalog, search, anime metadata, and episode video sources.

## Run

```bash
npm install
npm run build
TYPENX_VIDEO_LIBRARY_FILE=./videos.example.json PORT=8790 npm start
```

Routes:

- `GET /health`
- `GET /manifest`
- `POST /catalog`
- `POST /search`
- `GET /anime/:id`
- `POST /videos`

## Library File

Copy `videos.example.json` to `videos.local.json`, replace the sample URLs with your own self-hosted MP4/HLS/DASH URLs, then run:

```bash
TYPENX_VIDEO_LIBRARY_FILE=./videos.local.json npm start
```

The addon does not upload, mirror, or transcode media. It advertises URLs that you control.
