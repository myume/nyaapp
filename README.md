# Nyaapp

The Nyaa App (Nyaapp) is an all in one client for Nyaa that allows you to browse, download, and read literature.

## Features

- Enhanced browsing experience for literature on Nyaa
- Download literature locally directly from the client
- View detailed manga metadata
- Customizable reader with support for cbz format
- Support for single page, double page, long strip reader views
- Reading progress tracking
- Local library management

The idea is that this would be a [Mihon](https://github.com/mihonapp/mihon) or Tachiyomi for downloading locally. 

Obviously I still need to figure out the tracking aspect.

## Screenshots 

<img width="3810" height="2073" alt="image" src="https://github.com/user-attachments/assets/0fa26685-b133-42ff-9504-00297d1e5f60" />
<img width="3810" height="2073" alt="image" src="https://github.com/user-attachments/assets/52369dd2-130f-4a29-9b32-8acecd76e1b0" />
<img width="3810" height="2073" alt="image" src="https://github.com/user-attachments/assets/9a0a815d-7915-40d6-b85f-b7dfec3f4f81" />
<img width="3810" height="2073" alt="image" src="https://github.com/user-attachments/assets/46d189ab-7eef-4160-824e-6d92a6f5b63a" />
<img width="3810" height="2073" alt="image" src="https://github.com/user-attachments/assets/bb30e84e-3799-4bbd-b529-21af50974968" />
<img width="3810" height="2073" alt="image" src="https://github.com/user-attachments/assets/825ef117-d071-406e-8f9c-803657c9437f" />
<img width="3807" height="2064" alt="image" src="https://github.com/user-attachments/assets/53b0d43c-ae98-4455-a761-e2d4eca53e1f" />
<img width="3810" height="2073" alt="image" src="https://github.com/user-attachments/assets/9001d858-2b5c-499e-8d6d-ef42f3b3cac8" />

## Getting Started

### Requirements
- Node.js and npm
- Rust and Cargo
- Tauri v2 + [CLI](https://v2.tauri.app/reference/cli/)

If you are on nix, there is a flake with all dev dependencies in it.

Otherwise, you will need to download them yourself. See [tauri docs](https://v2.tauri.app/start/prerequisites/) for more info.

These docs assume you have the tauri cli installed. Substitute the tauri command for however you run the tauri cli.

### Install Dependencies

To install dependencies

```bash
cd src
npm install
```

### Development

To start development server:

```bash
tauri dev
```

if you are using the flake

```bash
just
```

### Building

To build the application for production use run

```bash
tauri build
```

## Download

idk, distributing software is hard man. just build it yourself until i figure this part out.

## Data Attribution

All metadata is provided by [Mangabaka](https://mangabaka.dev/database) and its underlying providers: 
- [AniList](https://anilist.co/terms)
- [Anime News Network](https://www.animenewsnetwork.com/copyright-policy)
- [Kitsu](https://kitsu.app/terms)
- [MyAnimeList](https://myanimelist.net/about/terms_of_use)
- [Mangadex](https://mangadex.org/compliance/terms)
- [MangaUpdates](https://www.mangaupdates.com/site/faq/7)
- [Wikidata](https://www.wikidata.org/wiki/Special:MyLanguage/Project:General_disclaimer)

## Disclaimer
- All media, text, images, videos, or other content accessed through the Nyaapp is provided by third‑party services, websites, or APIs.
- Nyaapp does not host, own, or claim ownership of any of the content it retrieves, displays, or downloads.
- Nyaapp merely scrapes, reads, or downloads data that is already publicly available.
