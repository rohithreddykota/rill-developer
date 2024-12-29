use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
};

#[warn(dead_code)]
pub mod org_projects;

// load_data loads if exists or fetches data from the server
pub async fn load_data<T, F, Fut>(path: &str, fetch: F) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    if fs::metadata(path).is_ok() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        log::info!("Loaded data from {}", path);
        Ok(data)
    } else {
        let data = fetch().await?;
        fs::create_dir_all("data")?;
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &data)?;
        log::info!("Fetched data and saved to {}", path);
        Ok(data)
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    org_projects::org_projects().await?;
    Ok(())
}
