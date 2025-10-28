# Nyaapp

The Nyaa App (Nyaapp) is an all in one client for Nyaa that allows you to
browse, download, and read literature.

[![CI](https://github.com/myume/nyaapp/actions/workflows/CI.yml/badge.svg)](https://github.com/myume/nyaapp/actions/workflows/CI.yml)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/myume/nyaapp)](https://github.com/myume/nyaapp/releases)

## Features

- Enhanced browsing experience for literature on Nyaa
- Download literature locally directly from the client
- View detailed manga metadata
- Customizable reader with support for cbz format
- Support for single page, double page, long strip reader views
- Reading progress tracking
- Local library management

The idea is that this would be a [Mihon](https://github.com/mihonapp/mihon) or
Tachiyomi for downloading locally.

Obviously I still need to figure out the tracking aspect.

## Screenshots
| <img src="https://github.com/user-attachments/assets/0fa26685-b133-42ff-9504-00297d1e5f60" width="800"/> | <img src="https://github.com/user-attachments/assets/52369dd2-130f-4a29-9b32-8acecd76e1b0" width="800"/> |
| :---: | :---: |
| <img src="https://github.com/user-attachments/assets/9a0a815d-7915-40d6-b85f-b7dfec3f4f81" width="800"/> | <img src="https://github.com/user-attachments/assets/46d189ab-7eef-4160-824e-6d92a6f5b63a" width="800"/> |
| <img src="https://github.com/user-attachments/assets/bb30e84e-3799-4bbd-b529-21af50974968" width="800"/> | <img src="https://github.com/user-attachments/assets/825ef117-d071-406e-8f9c-803657c9437f" width="800"/> |
| <img src="https://github.com/user-attachments/assets/53b0d43c-ae98-4455-a761-e2d4eca53e1f" width="800"/> | <img src="https://github.com/user-attachments/assets/9001d858-2b5c-499e-8d6d-ef42f3b3cac8" width="800"/> |

## Getting Started

### Requirements

- Node.js and npm
- Rust and Cargo

If you are on nix, there is a flake with the dev environment.

### Install Dependencies

To install dependencies

```bash
npm install
```

### Development

To start development server:

```bash
npm run tauri dev

# or if you have just installed
just
```

We are using SQLx which requires the DATABASE_URL env var to be set for local
developement. This step is optional. If you don't want to download the db
locally for type checking, you can just set `SQLX_OFFLINE=true`.

There is a script in the justfile which downloads the metadata db and sets the
var. If you have [just](https://github.com/casey/just) installed, you can use
the following command, otherwise you will need to do this manually.

```bash
just pull-meta
```

### Building

To build the application for production use run

```bash
npm run tauri build
```

## Download

Check out the latest [release](https://github.com/myume/nyaapp/releases). If
your platform doesn't have prebuilt binaries, you may need to build the
application yourself.

There is a provided flake which packages the application. You can use the flake
as an input in your config as so:

```nix
{
  ...

  inputs = {
    ...
    nyaapp = {
      url = "github:myume/nyaapp";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  ...
}
```

And then you can declare the package with

```nix
inputs.nyaapp.packages.${pkgs.system}.default
```

## Data Attribution

All metadata is provided by [Mangabaka](https://mangabaka.dev/database) and its
underlying providers:

- [AniList](https://anilist.co/terms)
- [Anime News Network](https://www.animenewsnetwork.com/copyright-policy)
- [Kitsu](https://kitsu.app/terms)
- [MyAnimeList](https://myanimelist.net/about/terms_of_use)
- [Mangadex](https://mangadex.org/compliance/terms)
- [MangaUpdates](https://www.mangaupdates.com/site/faq/7)
- [Wikidata](https://www.wikidata.org/wiki/Special:MyLanguage/Project:General_disclaimer)

## Disclaimer

- All media, text, images, videos, or other content accessed through the Nyaapp
  is provided by third‑party services, websites, or APIs.
- Nyaapp does not host, own, or claim ownership of any of the content it
  retrieves, displays, or downloads.
- Nyaapp merely scrapes, reads, or downloads data that is already publicly
  available.
