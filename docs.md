# See `ffmpeg` command to convert `.mp4` video message to mandatory format for video stickers  
```bash
ffmpeg -y -t 00:00:03 -i <source>.mp4 -i mask.png -filter_complex "
[1:v]alphaextract[alf];
[0:v][alf]alphamerge[out];
[out]scale=512:512
" -an -vb 0.7M -c:v libvpx-vp9 output.webm
```
where: 
- `-t` - makes video no longer of 3sec
- `-i <source>` - connects sources `[0:v]` and `[1:v]`
- `-filter_complex` - extracts alpha channel from mask source and applies it to main source, then upscales video to 512x512px
- `-an` - disables audio
- `-vb` - sets encoding bitrate
- `-c:v libvpx-vp9` - sets webm vp9 codec 

# Telegram bot
```bash
cargo new video_stickers
cd video_stickers
```
`./.env`
```
TELOXIDE_TOKEN=<BOT_TOKEN>
```
`./Cargo.toml`
```toml
[package]
name = "video_stickers"
version = "0.1.0"
edition = "2021"

[dependencies]
teloxide = { version = "0.10", features = [
  "macros",
  "auto-send",
  "ctrlc_handler",
  "sqlite-storage",
] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tokio = { version = "1.8", features = ["rt-multi-thread", "macros"] }
dotenv = "0.15"
anyhow = "1.0"
serde_json = "1.0"
serde = { version = "1.0", features = ["derive", "rc"] }
strum = { version = "0.24", features = ["derive"] }
async-trait = "0.1.56"
url = "2.2.2"
toml = "0.5.9"
```
