use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

static APP_USER_AGENT: &str =
  concat!("QALAM", "/", env!("CARGO_PKG_VERSION"),);

static DOWNLOAD_LOCATION: &str =
  "https://api.github.com/repos/uzinfocom-org/korrektor-dict/releases/latest";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Asset {
  name: String,
  browser_download_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GitHub {
  id: u32,
  tag_name: String,
  assets: Vec<Asset>,
}

async fn download_link(link: &str) -> GitHub {
  // Get info about latest builds
  let client = reqwest::Client::builder()
    .user_agent(APP_USER_AGENT)
    .build()
    .expect("Couldn't build the client");

  let target: GitHub = client
    .get(link)
    .send()
    .await
    .expect("Problems with Internet connectivity!")
    .json()
    .await
    .expect("Can't convert source into json!");
  
  target
}

async fn download(location: String) -> File {
  let mut dumpfile = tempfile::tempfile().unwrap();
  
  // let resp = reqwest::get(file.browser_download_url.to_string())
  
  let resp = reqwest::get(location)
    .await
    .expect("Problems with Internet connectivity!")
    .bytes()
    .await
    .expect("Can't convert source into bytes!");
  
  std::io::copy(&mut resp.as_ref(), &mut dumpfile)
    .expect("Failed to copy content");
  
  dumpfile
}

fn extract(temp: File, out: String) {
      let mut zip = zip::ZipArchive::new(temp).unwrap();

    // Extract files inside folder
    zip
      .extract(out)
      .expect("Couldn't extract zip file...");

}

pub async fn bootstrap() {
  let link = download_link(DOWNLOAD_LOCATION).await;
  
  for file in link.assets {
    let chunk = download(file.browser_download_url).await;
    extract(chunk, file.name.replace(".zip", ""));
  }
  
  println!("Assets has been updated successfully!")
}