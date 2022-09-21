//! ```bash
//! ffmpeg -y -t 00:00:03 -i <source>.mp4 -i mask.png -filter_complex "
//! [1:v]alphaextract[alf];
//! [0:v][alf]alphamerge[out];
//! [out]scale=512:512
//! " -an -vb 0.7M -c:v libvpx-vp9 output.webm
//! ```

use std::process::Command;
use tokio::fs;

const FILTER_COMPLEX: &str = r#"
[1:v]alphaextract[alf];
[0:v][alf]alphamerge[out];
[out]scale=512:512
"#;

pub async fn run(file_id: &str) -> anyhow::Result<String> {
    fs::create_dir_all("./dest").await?;

    tracing::trace!("starting ffmpeg for file: {}", file_id);

    let out = {
        let file_id = file_id.to_owned();
        tokio::task::spawn_blocking(move || {
            let cmd = Command::new("ffmpeg")
                .arg("-y")
                .arg("-t")
                .arg("00:00:02.950")
                .arg("-i")
                .arg(format!("./source/{}.mp4", file_id))
                .arg("-i")
                .arg("./mask/mask.png")
                .arg("-filter_complex")
                .arg(FILTER_COMPLEX)
                .arg("-an")
                .arg("-vb")
                .arg("0.7M")
                .arg("-c:v")
                .arg("libvpx-vp9")
                .arg(format!("./dest/{}.webm", file_id))
                .spawn()
                .expect("ffmpeg command failed to start");

            cmd.wait_with_output()
        })
        .await
        .expect("failed to wait for ffmpeg algo")
    };

    tracing::trace!("ffmpeg output: {:?}", out);

    tracing::trace!("finished ffmpeg for file: {}", file_id);
    Ok(format!("./dest/{}.webm", file_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn run_ffmpeg() {
        run("AgAD9hwAAgfwSUk").await.unwrap();
    }

    #[test]
    fn filter_complex_concat() {
        let filter = format!("-filter_complex {}", FILTER_COMPLEX);
        dbg!(filter);
    }
}
