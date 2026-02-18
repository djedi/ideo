# ideo

A command-line tool for generating images with the [Ideogram v3 API](https://developer.ideogram.ai). Designed to be used standalone or embedded in scripts and automation pipelines.

```bash
ideo "A watercolor painting of a mountain lake"
```

## Prerequisites

- An [Ideogram API key](https://developer.ideogram.ai/ideogram-api/api-setup)

## Installation

### With Homebrew

```bash
brew tap djedi/tap
brew install ideo
```

### From source

Requires [Rust](https://rustup.rs/) (1.85+).

```bash
git clone https://github.com/djedi/ideo.git
cd ideo
cargo install --path .
```

### Direct from GitHub

```bash
cargo install --git https://github.com/djedi/ideo.git
```

## Setup

Set your Ideogram API key as an environment variable. Add this to your `~/.zshrc`, `~/.bashrc`, or equivalent:

```bash
export IDEOGRAM_API_KEY="your-api-key-here"
```

You can get an API key from the [Ideogram developer dashboard](https://developer.ideogram.ai/ideogram-api/api-setup).

## Usage

```
ideo [options] <prompt>
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-o, --output FILE` | Output file path | `ideo_<timestamp>.png` |
| `-a, --aspect RATIO` | Aspect ratio | `1x1` |
| `-s, --speed SPEED` | Rendering speed | `TURBO` |
| `-n, --num NUM` | Number of images | `1` |
| `--style TYPE` | Style type | — |
| `--negative TEXT` | Negative prompt | — |
| `--seed NUM` | Seed for reproducibility | — |
| `--magic-prompt MODE` | Prompt enhancement | — |
| `-h, --help` | Show help | — |

### Aspect ratios

`1x1`, `16x9`, `9x16`, `4x3`, `3x4`, `3x2`, `2x3`, `4x5`, `5x4`, `16x10`, `10x16`, `2x1`, `1x2`, `3x1`, `1x3`

### Rendering speeds

| Speed | Description |
|-------|-------------|
| `FLASH` | Fastest, lower quality |
| `TURBO` | Fast, good quality (default) |
| `DEFAULT` | Balanced |
| `QUALITY` | Slowest, highest quality |

### Style types

`AUTO`, `GENERAL`, `REALISTIC`, `DESIGN`, `FICTION`

### Magic prompt

Controls whether Ideogram automatically enhances your prompt:

- `AUTO` — let the API decide
- `ON` — always enhance
- `OFF` — use your prompt exactly as written

## Examples

Generate a simple image:

```bash
ideo "A cat sitting on a windowsill at sunset"
```

Specify an output path and widescreen aspect ratio:

```bash
ideo -o hero.png -a 16x9 "Minimalist tech blog header with abstract shapes"
```

High quality realistic photo:

```bash
ideo -s QUALITY --style REALISTIC "Portrait of a golden retriever in a field"
```

Generate multiple variations:

```bash
ideo -n 4 --seed 42 "Logo design for a coffee shop called Ember"
```

Exclude elements with a negative prompt:

```bash
ideo --negative "text, watermark, blurry" "Product photo of a ceramic mug"
```

## Scripting

`ideo` is built for scripting. File paths are printed to **stdout** while status messages go to **stderr**, so you can capture output cleanly:

```bash
# Capture the generated image path
image=$(ideo -a 16x9 "Blog header about machine learning")
echo "Image saved to: $image"

# Use in a blog post pipeline
featured_image=$(ideo -a 16x9 -o "posts/my-post/featured.png" "Abstract illustration of neural networks")

# Generate and immediately open (macOS)
open "$(ideo "A pencil sketch of a treehouse")"

# Batch generate from a file of prompts
while IFS= read -r prompt; do
  ideo -o "output/$(echo "$prompt" | tr ' ' '_' | head -c 40).png" "$prompt"
done < prompts.txt
```

## License

MIT
