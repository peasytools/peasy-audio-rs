# peasy-audio

[![crates.io](https://img.shields.io/crates/v/peasy-audio)](https://crates.io/crates/peasy-audio)
[![docs.rs](https://docs.rs/peasy-audio/badge.svg)](https://docs.rs/peasy-audio)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://agentgif.com/badge/crates/peasy-audio/version.svg)](https://crates.io/crates/peasy-audio)
[![GitHub stars](https://agentgif.com/badge/github/peasytools/peasy-audio-rs/stars.svg)](https://github.com/peasytools/peasy-audio-rs)

Async Rust client for the [PeasyAudio](https://peasyaudio.com) API — analyze BPM, calculate bitrate, and convert audio formats. Built with reqwest, serde, and tokio.

Built from [PeasyAudio](https://peasyaudio.com), a comprehensive audio toolkit offering free online tools for analyzing tempo, calculating file sizes, comparing audio formats, and converting between MP3, WAV, FLAC, OGG, and AAC. The site includes in-depth guides on lossless vs. lossy audio encoding, format comparison charts, and a glossary covering concepts from bitrate and sample rate to audio codecs and clipping.

> **Try the interactive tools at [peasyaudio.com](https://peasyaudio.com)** — [Audio BPM Analyzer](https://peasyaudio.com/audio/audio-bpm/), [Audio Frequency Calculator](https://peasyaudio.com/audio/audio-freq/), [Audio File Size Calculator](https://peasyaudio.com/audio/audio-filesize/), and more.

<p align="center">
  <a href="https://agentgif.com/qKpEKPwD"><img src="https://media.agentgif.com/qKpEKPwD.gif" alt="peasy-audio demo — audio BPM analysis and format conversion tools in Rust terminal" width="800"></a>
</p>

## Table of Contents

- [Install](#install)
- [Quick Start](#quick-start)
- [What You Can Do](#what-you-can-do)
  - [Audio Analysis Tools](#audio-analysis-tools)
  - [Browse Reference Content](#browse-reference-content)
  - [Search and Discovery](#search-and-discovery)
- [API Client](#api-client)
  - [Available Methods](#available-methods)
- [Learn More About Audio Tools](#learn-more-about-audio-tools)
- [Also Available](#also-available)
- [Peasy Developer Tools](#peasy-developer-tools)
- [License](#license)

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

## What You Can Do

### Audio Analysis Tools

Digital audio is represented as a series of samples captured at a fixed rate — CD-quality audio uses 44,100 samples per second (44.1 kHz) with 16-bit depth, producing 1,411 kbps of uncompressed data. Lossy codecs like MP3 and AAC reduce this dramatically (128-320 kbps) by discarding inaudible frequencies using psychoacoustic models, while lossless codecs like FLAC compress without any data loss. PeasyAudio provides calculators and analysis tools for understanding these encoding parameters.

| Tool | Slug | Description |
|------|------|-------------|
| BPM Analyzer | `audio-bpm` | Calculate beats per minute for tempo analysis |
| Frequency Calculator | `audio-freq` | Compute audio frequency values and wavelengths |
| File Size Calculator | `audio-filesize` | Estimate file sizes for different bitrate and duration combinations |

```rust
use peasy_audio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Get the BPM analyzer tool for tempo detection
    let tool = client.get_tool("audio-bpm").await?;
    println!("Tool: {}", tool.name);              // Audio BPM analyzer name
    println!("Description: {}", tool.description); // How BPM detection works

    // List all available audio tools with pagination
    let opts = peasy_audio::ListOptions {
        page: Some(1),
        limit: Some(20),
        ..Default::default()
    };
    let tools = client.list_tools(&opts).await?;
    println!("Total audio tools available: {}", tools.count);

    Ok(())
}
```

Learn more: [Audio BPM Analyzer](https://peasyaudio.com/audio/audio-bpm/) · [Audio Format Comparison](https://peasyaudio.com/guides/audio-format-comparison/) · [Convert Between Audio Formats](https://peasyaudio.com/guides/convert-between-audio-formats/)

### Browse Reference Content

PeasyAudio includes a comprehensive glossary of audio engineering terminology and practical guides for common workflows. The glossary covers foundational concepts like bitrate (the number of bits processed per second, determining audio quality and file size), sample rate (how many times per second the audio signal is measured), WAV (Microsoft's uncompressed audio container), and FLAC (Free Lossless Audio Codec, the open-source standard for archival-quality audio).

| Term | Description |
|------|-------------|
| [Bitrate](https://peasyaudio.com/glossary/bitrate/) | Bits per second — determines audio quality and file size |
| [Sample Rate](https://peasyaudio.com/glossary/sample-rate/) | Samples per second — 44.1 kHz (CD), 48 kHz (video), 96 kHz (hi-res) |
| [WAV](https://peasyaudio.com/glossary/wav/) | Waveform Audio File Format — uncompressed PCM container |
| [FLAC](https://peasyaudio.com/glossary/flac/) | Free Lossless Audio Codec — open-source lossless compression |

```rust
use peasy_audio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Browse the audio glossary for encoding and format terminology
    let glossary = client.list_glossary(&peasy_audio::ListOptions {
        search: Some("bitrate".into()), // Search for audio encoding concepts
        ..Default::default()
    }).await?;
    for term in &glossary.results {
        println!("{}: {}", term.term, term.definition);
    }

    // Read a guide comparing lossless vs lossy audio formats
    let guide = client.get_guide("audio-format-comparison").await?;
    println!("Guide: {} (Level: {})", guide.title, guide.audience_level);

    Ok(())
}
```

Learn more: [Audio Glossary](https://peasyaudio.com/glossary/) · [Audio Format Comparison](https://peasyaudio.com/guides/audio-format-comparison/) · [Convert Between Audio Formats](https://peasyaudio.com/guides/convert-between-audio-formats/)

### Search and Discovery

The API supports full-text search across all content types — tools, glossary terms, guides, use cases, and format documentation. Search results are grouped by content type, making it easy to find the right tool or reference for any audio workflow.

```rust
use peasy_audio::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Search across all audio content — tools, glossary, guides, and formats
    let results = client.search("convert flac", Some(20)).await?;
    println!("Found {} tools, {} glossary terms, {} guides",
        results.results.tools.len(),
        results.results.glossary.len(),
        results.results.guides.len(),
    );

    // Discover format conversion paths — what can WAV convert to?
    let conversions = client.list_conversions(&peasy_audio::ListConversionsOptions {
        source: Some("wav".into()), // Find all formats WAV can be converted to
        ..Default::default()
    }).await?;
    for c in &conversions.results {
        println!("{} -> {}", c.source_format, c.target_format);
    }

    Ok(())
}
```

Learn more: [REST API Docs](https://peasyaudio.com/developers/) · [All Audio Tools](https://peasyaudio.com/)

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

## Learn More About Audio Tools

- **Tools**: [Audio BPM Analyzer](https://peasyaudio.com/audio/audio-bpm/) · [Audio Frequency Calculator](https://peasyaudio.com/audio/audio-freq/) · [Audio File Size Calculator](https://peasyaudio.com/audio/audio-filesize/) · [All Tools](https://peasyaudio.com/)
- **Guides**: [Audio Format Comparison](https://peasyaudio.com/guides/audio-format-comparison/) · [Convert Between Audio Formats](https://peasyaudio.com/guides/convert-between-audio-formats/) · [All Guides](https://peasyaudio.com/guides/)
- **Glossary**: [Bitrate](https://peasyaudio.com/glossary/bitrate/) · [Sample Rate](https://peasyaudio.com/glossary/sample-rate/) · [WAV](https://peasyaudio.com/glossary/wav/) · [FLAC](https://peasyaudio.com/glossary/flac/) · [All Terms](https://peasyaudio.com/glossary/)
- **Formats**: [All Formats](https://peasyaudio.com/formats/)
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
