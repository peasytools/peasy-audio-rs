# peasy-audio

[![crates.io](https://img.shields.io/crates/v/peasy-audio)](https://crates.io/crates/peasy-audio)
[![docs.rs](https://docs.rs/peasy-audio/badge.svg)](https://docs.rs/peasy-audio)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Async Rust client for the [PeasyAudio](https://peasyaudio.com) API — convert, trim, merge, and normalize audio files across MP3, WAV, OGG, FLAC, and AAC formats. Built with reqwest, serde, and tokio.

Built from [PeasyAudio](https://peasyaudio.com), a free online audio toolkit with tools for converting, trimming, merging, and normalizing audio in all major formats.

> **Try the interactive tools at [peasyaudio.com](https://peasyaudio.com)** — [Audio Tools](https://peasyaudio.com/), [Audio Glossary](https://peasyaudio.com/glossary/), [Audio Guides](https://peasyaudio.com/guides/)

## Install

```toml
[dependencies]
peasy-audio = "0.2.0"
tokio = { version = "1", features = ["full"] }
```

Or via cargo:

```bash
cargo add peasy-audio
```

## Quick Start

```rust
use peasy_audio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // List available audio tools
    let tools = client.list_tools(&Default::default()).await?;
    for tool in &tools.results {
        println!("{}: {}", tool.name, tool.description);
    }

    Ok(())
}
```

## API Client

The client wraps the [PeasyAudio REST API](https://peasyaudio.com/developers/) with strongly-typed Rust structs using serde deserialization.

```rust
use peasy_audio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // Or with a custom base URL:
    // let client = Client::with_base_url("https://custom.example.com");

    // List tools with filters
    let opts = peasy_audio::ListOptions {
        page: Some(1),
        limit: Some(10),
        search: Some("convert".into()),
        ..Default::default()
    };
    let tools = client.list_tools(&opts).await?;

    // Get a specific tool
    let tool = client.get_tool("audio-convert").await?;
    println!("{}: {}", tool.name, tool.description);

    // Search across all content
    let results = client.search("convert", Some(20)).await?;
    println!("Found {} tools", results.results.tools.len());

    // Browse the glossary
    let glossary = client.list_glossary(&peasy_audio::ListOptions {
        search: Some("mp3".into()),
        ..Default::default()
    }).await?;
    for term in &glossary.results {
        println!("{}: {}", term.term, term.definition);
    }

    // Discover guides
    let guides = client.list_guides(&peasy_audio::ListGuidesOptions {
        category: Some("audio".into()),
        ..Default::default()
    }).await?;
    for guide in &guides.results {
        println!("{} ({})", guide.title, guide.audience_level);
    }

    // List format conversions
    let conversions = client.list_conversions(&peasy_audio::ListConversionsOptions {
        source: Some("wav".into()),
        ..Default::default()
    }).await?;

    Ok(())
}
```

### Available Methods

| Method | Description |
|--------|-------------|
| `list_tools(&opts)` | List tools (paginated, filterable) |
| `get_tool(slug)` | Get tool by slug |
| `list_categories(&opts)` | List tool categories |
| `list_formats(&opts)` | List file formats |
| `get_format(slug)` | Get format by slug |
| `list_conversions(&opts)` | List format conversions |
| `list_glossary(&opts)` | List glossary terms |
| `get_glossary_term(slug)` | Get glossary term |
| `list_guides(&opts)` | List guides |
| `get_guide(slug)` | Get guide by slug |
| `list_use_cases(&opts)` | List use cases |
| `search(query, limit)` | Search across all content |
| `list_sites()` | List Peasy sites |
| `openapi_spec()` | Get OpenAPI specification |

Full API documentation at [peasyaudio.com/developers/](https://peasyaudio.com/developers/).
OpenAPI 3.1.0 spec: [peasyaudio.com/api/openapi.json](https://peasyaudio.com/api/openapi.json).

## Learn More

- **Tools**: [Audio Convert](https://peasyaudio.com/tools/audio-convert/) · [Audio Trim](https://peasyaudio.com/tools/audio-trim/) · [All Tools](https://peasyaudio.com/)
- **Guides**: [How to Convert Audio](https://peasyaudio.com/guides/convert-audio/) · [All Guides](https://peasyaudio.com/guides/)
- **Glossary**: [What is MP3?](https://peasyaudio.com/glossary/mp3/) · [All Terms](https://peasyaudio.com/glossary/)
- **Formats**: [WAV](https://peasyaudio.com/formats/wav/) · [All Formats](https://peasyaudio.com/formats/)
- **API**: [REST API Docs](https://peasyaudio.com/developers/) · [OpenAPI Spec](https://peasyaudio.com/api/openapi.json)

## Also Available

| Language | Package | Install |
|----------|---------|---------|
| **Python** | [peasy-audio](https://pypi.org/project/peasy-audio/) | `pip install "peasy-audio[all]"` |
| **TypeScript** | [peasy-audio](https://www.npmjs.com/package/peasy-audio) | `npm install peasy-audio` |
| **Go** | [peasy-audio-go](https://pkg.go.dev/github.com/peasytools/peasy-audio-go) | `go get github.com/peasytools/peasy-audio-go` |
| **Ruby** | [peasy-audio](https://rubygems.org/gems/peasy-audio) | `gem install peasy-audio` |

## Peasy Developer Tools

Part of the [Peasy Tools](https://peasytools.com) open-source developer ecosystem.

| Package | PyPI | npm | crates.io | Description |
|---------|------|-----|-----------|-------------|
| peasy-pdf | [PyPI](https://pypi.org/project/peasy-pdf/) | [npm](https://www.npmjs.com/package/peasy-pdf) | [crate](https://crates.io/crates/peasy-pdf) | PDF merge, split, rotate, compress — [peasypdf.com](https://peasypdf.com) |
| peasy-image | [PyPI](https://pypi.org/project/peasy-image/) | [npm](https://www.npmjs.com/package/peasy-image) | [crate](https://crates.io/crates/peasy-image) | Image resize, crop, convert, compress — [peasyimage.com](https://peasyimage.com) |
| **peasy-audio** | [PyPI](https://pypi.org/project/peasy-audio/) | [npm](https://www.npmjs.com/package/peasy-audio) | [crate](https://crates.io/crates/peasy-audio) | Audio trim, merge, convert, normalize — [peasyaudio.com](https://peasyaudio.com) |
| peasy-video | [PyPI](https://pypi.org/project/peasy-video/) | [npm](https://www.npmjs.com/package/peasy-video) | [crate](https://crates.io/crates/peasy-video) | Video trim, resize, thumbnails, GIF — [peasyvideo.com](https://peasyvideo.com) |
| peasy-css | [PyPI](https://pypi.org/project/peasy-css/) | [npm](https://www.npmjs.com/package/peasy-css) | [crate](https://crates.io/crates/peasy-css) | CSS minify, format, analyze — [peasycss.com](https://peasycss.com) |
| peasy-compress | [PyPI](https://pypi.org/project/peasy-compress/) | [npm](https://www.npmjs.com/package/peasy-compress) | [crate](https://crates.io/crates/peasy-compress) | ZIP, TAR, gzip compression — [peasytools.com](https://peasytools.com) |
| peasy-document | [PyPI](https://pypi.org/project/peasy-document/) | [npm](https://www.npmjs.com/package/peasy-document) | [crate](https://crates.io/crates/peasy-document) | Markdown, HTML, CSV, JSON conversion — [peasyformats.com](https://peasyformats.com) |
| peasytext | [PyPI](https://pypi.org/project/peasytext/) | [npm](https://www.npmjs.com/package/peasytext) | [crate](https://crates.io/crates/peasytext) | Text case conversion, slugify, word count — [peasytext.com](https://peasytext.com) |

## License

MIT
