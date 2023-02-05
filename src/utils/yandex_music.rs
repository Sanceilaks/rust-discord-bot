use std::{
    io::{Write, BufReader},
    process::{Command, Stdio}, mem, fmt::format,
};

use futures::{StreamExt, io::{Read, Cursor}};
use regex::Regex;
use reqwest::{self, header, Client};
use serde::{Deserialize, Serialize};
use songbird::{input::{children_to_reader, Reader, ChildContainer}, constants::{STEREO_FRAME_SIZE, CHILD_BUFFER_LEN}};
use tokio::task;
use streamcatcher::{Catcher, TxCatcher};

#[derive(Debug, Deserialize, Serialize)]
struct InvocationInfo {
    #[serde(rename = "exec-duration-millis")]
    exec_duration_millis: i32,
    hostname: String,

    #[serde(rename = "req-id")]
    req_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TrackInfoResult {
    #[serde(rename = "bitrateInKbps")]
    bitrate_in_kbps: i32,
    codec: String,
    direct: bool,

    #[serde(rename = "downloadInfoUrl")]
    download_info_url: String,
    gain: bool,
    preview: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct TrackInfo {
    #[serde(rename = "invocationInfo")]
    invocation_info: InvocationInfo,
    result: Vec<TrackInfoResult>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "download-info")]
struct DownloadInfo {
    host: String,
    path: String,
    ts: String,
    region: i32,
    s: String,
}

pub async fn get_track(url: &str) -> Result<songbird::input::Input, String> {
    let id = match url.starts_with("http") {
        true => {
            let argument = url;
            Regex::new(r"(\d+)")
                .unwrap()
                .captures_iter(&argument)
                .nth(1)
                .and_then(|cap| Some(cap.get(0).unwrap()))
                .unwrap()
                .as_str()
                .into()
        }
        false => url,
    };
    let token = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN must be set");

    let mut auth_value = header::HeaderValue::from_str(&token).unwrap();
    auth_value.set_sensitive(true);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", auth_value);
    headers.insert("User-Agent", "Windows 10".parse().unwrap());

    let client = Client::builder()
        .default_headers(headers.to_owned())
        .build()
        .unwrap();

    let result = client
        .get(std::format!(
            "https://api.music.yandex.net/tracks/{}/download-info?can_use_streaming=false",
            &id
        ))
        .send()
        .await
        .unwrap();

    if result.status().is_success() {
        let download_info = result.json::<TrackInfo>().await.unwrap();
        let download_urls: Vec<&String> = download_info
            .result
            .iter()
            .filter(|x| x.codec == "mp3")
            .filter(|x| x.bitrate_in_kbps >= 320)
            .map(|x| &x.download_info_url)
            .collect();
		
        if let Some(download_url) = download_urls.first() {
            let result = client
                .get(download_url.to_owned())
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            let download_data: DownloadInfo =
                serde_xml_rs::from_str(std::str::from_utf8(result.as_bytes()).unwrap()).unwrap();

            let seed = md5::compute(std::format!(
                "XGRlBW9FXlekgbPrRHuSiA{}{}",
                &download_data.path[1..download_data.path.len()],
                &download_data.s
            ));

            let mp3_url = format!(
                "https://{}/get-mp3/{:x}/{}{}",
                download_data.host.to_owned(),
                seed,
                download_data.ts,
                download_data.path.to_owned()
            );
			
			let mut curl = Command::new("curl")
				.arg(format!("{}", &mp3_url))
				.arg("--output")
				.arg("-")
				.stdin(Stdio::null())
				.stdout(Stdio::piped())
				.stderr(Stdio::null())
				.spawn()
				.unwrap();
			
			let curl_cout = curl.stdout.take().unwrap();

			let ffmpeg_args = [
				"-f",
				"s16le",
				"-ac",
				"2",
				"-ar",
				"48000",
				"-acodec",
				"pcm_f32le",
				"-",
			];

			let ffmpeg = Command::new("ffmpeg")
				.arg("-i")
				.arg("-")
				.args(&ffmpeg_args)
				.stdin(curl_cout)
				.stderr(Stdio::null())
				.stdout(Stdio::piped())
				.spawn()
				.unwrap();

            return Ok(songbird::input::Input::new(
                true,
                children_to_reader::<f32>(vec![curl, ffmpeg]),
                songbird::input::Codec::FloatPcm,
                songbird::input::Container::Raw,
				Some(Default::default())
            ))
        }
    }

    Err(("Failed to get track info".to_string()).into())
}
