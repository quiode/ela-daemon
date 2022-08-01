use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use config::Config;
use tokio::time;

async fn send_ping(domain: &str, uuid: &str) {
    let url = format!("http://{}/api/ping/{}", domain, uuid);

    let client = reqwest::Client::new();

    client.post(&url).send().await.ok();
}

#[tokio::main]
async fn main() {
    let config_file_path: &Path = Path::new("/etc/ela/Settings.toml");

    // create uuid if no file exists
    if !config_file_path.exists() {
        let parent_path = config_file_path.parent().unwrap();
        fs::create_dir_all(parent_path).unwrap();

        let uuid = uuid::Uuid::new_v4().to_string();

        let mut file = File::create(config_file_path).unwrap();
        file.write_all(format!("UUID = \"{}\"", uuid).as_bytes())
            .unwrap();
        file.flush().unwrap();
    }

    // read values form configurations
    let settings = Config::builder()
        .set_default("DOMAIN", "ela.team-crystal.ch")
        .unwrap()
        .add_source(
            config::Environment::with_prefix("ELA")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        )
        .add_source(config::File::with_name(&config_file_path.to_str().unwrap()))
        .build()
        .unwrap();

    let uuid: String;

    // checks if uuid has been created, if not, create one
    if settings.get_string("UUID").is_ok() {
        uuid = settings.get_string("UUID").unwrap();
    } else {
        uuid = uuid::Uuid::new_v4().to_string();

        let mut file = File::create(config_file_path).unwrap();
        file.write_all(format!("UUID = \"{}\"", uuid).as_bytes())
            .unwrap();
        file.flush().unwrap();
    }

    let domain = settings.get_string("DOMAIN").unwrap();

    let mut interval = time::interval(time::Duration::from_secs(60 * 2));
    loop {
        interval.tick().await;
        send_ping(&domain, &uuid).await;
    }
}
