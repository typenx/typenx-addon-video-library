# Typenx Video Library Addon

A self-hosted video source backed by a JSON manifest of URLs you already control.

Sometimes you don't want a Plex or Jellyfin server in the loop - you just have files on a CDN, an S3 bucket, or a folder served by nginx, and you want them to show up in Typenx. That's what this addon is for. Point it at a JSON file with your shows and episodes, and [Typenx Core](https://github.com/typenx/typenx-core) will treat it as a regular video source: catalog, search, metadata, and direct stream URLs.

The addon does not upload, mirror, or transcode anything. It advertises URLs that you already host.

## Run

```bash
cargo build --release
TYPENX_VIDEO_LIBRARY_FILE=./videos.example.json PORT=8790 cargo run --release
```

## Library file

Copy `videos.example.json` to `videos.local.json`, replace the sample URLs with your own MP4, HLS, or DASH links, then run:

```bash
TYPENX_VIDEO_LIBRARY_FILE=./videos.local.json cargo run --release
```

The schema is intentionally small - open the example file and you'll see exactly the shape it expects.

## Routes

- `GET /health`
- `GET /manifest`
- `POST /catalog`
- `POST /search`
- `GET /anime/:id`
- `POST /videos`

## Validate

```bash
cargo fmt --check
cargo test
cargo build --release
```

## Wiring it into Typenx Core

```env
TYPENX_DEFAULT_ADDONS=http://127.0.0.1:8790
```

Useful next to a metadata addon: the metadata addon supplies posters, synopses, and tracking; this addon supplies the actual stream URL.
