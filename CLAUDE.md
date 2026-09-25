# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A Rust CLI that copies photos and videos from a camera folder into date-based folders (`<dest>/YYYY/MM/DD/<original filename>`), following Luminar 3's folder layout. The date comes from EXIF `DateTimeOriginal`. Files already at their destination path are skipped, so running it again is safe.

```
photo-importer [-d|--show-diff] <CAMERA_PATH> [TARGET_PATH]   # TARGET_PATH defaults to $HOME/Pictures
```

`--show-diff` is a dry run: it prints `src ---> dest` for each file that would be copied and copies nothing.

## Commands

```sh
cargo build --release
cargo run -- --show-diff /path/to/DCIM /tmp/out   # dry run against a real card
cargo test                                          # no tests exist yet
```

## Architecture

- `src/main.rs` — clap argument parsing, the list of accepted extensions (hard-coded and case-sensitive, so each case variant is listed separately), EXIF reading, and the copy loop. `get_date_path`/`parse_to_yymmdd` slice the EXIF datetime string (`YYYY-MM-DD ...`) by fixed byte offsets.
- `src/camera_dir.rs` — `CameraDir` recursively collects every file with an accepted extension under a root. Both the source and the destination are scanned with it.
- `src/raw.rs` — a fallback for RAW files that `kamadak-exif`'s `read_from_container` can't parse. It scans the first 64 KiB for one of two hard-coded embedded-JPEG/Exif APP1 marker byte sequences, then EXIF parsing is retried from that offset. To support a new RAW format that fails EXIF parsing, add its marker to `TIFF_MARKERS` and bump `TIFF_MARKET_TYPE_NUM`.

Things to know:
- Files with no readable EXIF are skipped without any message. If a file has EXIF but no `DateTimeOriginal`, `get_date_path(...).unwrap()` panics.
- The `target_dir_photos` HashSet in `main.rs` compares absolute paths from two different roots, so it almost never filters anything. What actually prevents duplicates is the `new_path.is_file()` check.

## CI / release binaries

On every push to `main`, `.github/workflows/build.yml` cross-compiles for linux-musl and windows-gnu on Ubuntu. A separate job on a macOS runner builds a universal (arm64 + x86_64) binary with `lipo`. Don't go back to the x86_64-only darwin build: it fails with "Bad CPU type" on Apple Silicon Macs without Rosetta. The workflow copies the binaries into `bin/` (`photo-importer`, `photo-importer.exe`, `photo-importer-mac`) and pushes them back to `main` as a commit titled `build`. Don't edit `bin/` by hand, and expect a CI `build` commit after each push. Pull before pushing again.
