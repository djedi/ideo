use std::io::Write;
use std::path::PathBuf;
use std::{env, fs, process};

use chrono::Local;
use clap::Parser;
use serde::Deserialize;
use serde_json::{Map, Value};

/// Generate images with the Ideogram v3 API.
///
/// File paths are printed to stdout (one per line).
/// Status messages go to stderr.
#[derive(Parser)]
#[command(name = "ideo", version)]
struct Cli {
    /// The prompt to generate an image from
    prompt: String,

    /// Output file path (default: ideo_<timestamp>.png)
    #[arg(short, long)]
    output: Option<String>,

    /// Aspect ratio: 1x1, 16x9, 9x16, 4x3, 3x4, etc.
    #[arg(short, long, default_value = "1x1")]
    aspect: String,

    /// Rendering speed: FLASH, TURBO, DEFAULT, QUALITY
    #[arg(short, long, default_value = "TURBO")]
    speed: String,

    /// Number of images to generate
    #[arg(short, long, default_value_t = 1)]
    num: u32,

    /// Style type: AUTO, GENERAL, REALISTIC, DESIGN, FICTION
    #[arg(long)]
    style: Option<String>,

    /// Negative prompt — what to exclude from the image
    #[arg(long)]
    negative: Option<String>,

    /// Random seed for reproducible generation
    #[arg(long)]
    seed: Option<u64>,

    /// Magic prompt mode: AUTO, ON, OFF
    #[arg(long)]
    magic_prompt: Option<String>,
}

#[derive(Deserialize)]
struct ApiResponse {
    data: Vec<ImageData>,
}

#[derive(Deserialize)]
struct ImageData {
    url: String,
}

fn main() {
    let cli = Cli::parse();

    let api_key = env::var("IDEOGRAM_API_KEY").unwrap_or_else(|_| {
        eprintln!("Error: IDEOGRAM_API_KEY environment variable is not set");
        process::exit(1);
    });

    // Build JSON payload
    let mut body = Map::new();
    body.insert("prompt".into(), Value::String(cli.prompt));
    body.insert("aspect_ratio".into(), Value::String(cli.aspect));
    body.insert("rendering_speed".into(), Value::String(cli.speed));
    body.insert(
        "num_images".into(),
        Value::Number(cli.num.into()),
    );

    if let Some(style) = cli.style {
        body.insert("style_type".into(), Value::String(style));
    }
    if let Some(negative) = cli.negative {
        body.insert("negative_prompt".into(), Value::String(negative));
    }
    if let Some(seed) = cli.seed {
        body.insert("seed".into(), Value::Number(seed.into()));
    }
    if let Some(magic) = cli.magic_prompt {
        body.insert("magic_prompt".into(), Value::String(magic));
    }

    eprintln!("Generating image...");

    let client = reqwest::blocking::Client::new();

    // Call API
    let response = client
        .post("https://api.ideogram.ai/v1/ideogram-v3/generate")
        .header("Api-Key", &api_key)
        .json(&body)
        .send()
        .unwrap_or_else(|e| {
            eprintln!("Error: request failed: {e}");
            process::exit(1);
        });

    let status = response.status();
    if !status.is_success() {
        let text = response.text().unwrap_or_default();
        eprintln!("Error: API returned HTTP {status}");
        // Try to pretty-print the error JSON
        if let Ok(json) = serde_json::from_str::<Value>(&text) {
            eprintln!("{}", serde_json::to_string_pretty(&json).unwrap());
        } else {
            eprintln!("{text}");
        }
        process::exit(1);
    }

    let api_response: ApiResponse = response.json().unwrap_or_else(|e| {
        eprintln!("Error: failed to parse API response: {e}");
        process::exit(1);
    });

    let image_count = api_response.data.len();

    // Download images
    for (i, image) in api_response.data.iter().enumerate() {
        let dest = match &cli.output {
            Some(output) if image_count == 1 => PathBuf::from(output),
            Some(output) => {
                let path = PathBuf::from(output);
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                let ext = path.extension().unwrap_or_default().to_string_lossy();
                let parent = path.parent().unwrap_or(std::path::Path::new(""));
                parent.join(format!("{stem}_{}.{ext}", i + 1))
            }
            None => {
                let ts = Local::now().format("%Y%m%d_%H%M%S");
                if image_count == 1 {
                    PathBuf::from(format!("ideo_{ts}.png"))
                } else {
                    PathBuf::from(format!("ideo_{ts}_{}.png", i + 1))
                }
            }
        };

        // Create parent directories if needed
        if let Some(parent) = dest.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).unwrap_or_else(|e| {
                    eprintln!("Error: could not create directory {}: {e}", parent.display());
                    process::exit(1);
                });
            }
        }

        let bytes = client
            .get(&image.url)
            .send()
            .and_then(|r| r.bytes())
            .unwrap_or_else(|e| {
                eprintln!("Error: failed to download image: {e}");
                process::exit(1);
            });

        fs::File::create(&dest)
            .and_then(|mut f| f.write_all(&bytes))
            .unwrap_or_else(|e| {
                eprintln!("Error: failed to write {}: {e}", dest.display());
                process::exit(1);
            });

        eprintln!("Saved: {}", dest.display());
        println!("{}", dest.display());
    }
}
