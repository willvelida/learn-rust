use std::collections::HashSet;
use std::result;
use std::sync::Arc;
use reqwest::Url;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

pub async fn run(seed: String, max_pages: usize) {
    let seed = normalize(&seed);
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let limit = Arc::new(Semaphore::new(8));
    let mut tasks: JoinSet<Vec<String>> = JoinSet::new();
    let mut fetched = 0;

    visited.lock().await.insert(seed.clone());
    tasks.spawn(crawl_one(seed, Arc::clone(&visited), Arc::clone(&limit)));
    fetched += 1;

    while let Some(result) = tasks.join_next().await {
        let found = result.unwrap_or_default();
        for url in found  {
            if fetched >= max_pages {
                break;
            }
            tasks.spawn(crawl_one(url, Arc::clone(&visited), Arc::clone(&limit)));
            fetched += 1;
        }
    }
}


async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

fn extract_links(html: &str, base: &str) -> Vec<String> {
    let origin = origin_of(base);
    html.match_indices("href=\"")
        .filter_map(|(i, _)| {
            let rest = &html[i + 6..];
            rest.find('"').map(|end| &rest[..end])
        })
        .map(|link| {
            if link.starts_with('/') {
                format!("{origin}{link}")
            } else {
                link.to_string()
            }
        })
        .filter(|link| link.starts_with(origin))
        .filter(|link| !is_asset(link))
        .map(|link| normalize(&link))
        .collect()

}

fn origin_of(url: &str) -> &str {
    match url.match_indices("/").nth(2) {
        Some((slash, _)) => &url[..slash],
        None => url,
    }
}

fn is_asset(url: &str) -> bool {
    const EXTS: [&str; 9] = [".css", ".js", ".ico", ".png", ".jpg", ".svg", ".xml", ".json", ".webp"];
    EXTS.iter().any(|ext| url.ends_with(ext))
}

fn normalize(url: &str) -> String {
    url.strip_suffix('/').unwrap_or(url).to_string()
}

async fn crawl_one(
    url: String,
    visited: Arc<Mutex<HashSet<String>>>,
    limit: Arc<Semaphore>,
) -> Vec<String> {
    let _permit = limit.acquire_owned().await.unwrap();

    let html = match fetch(&url).await {
        Ok(body) => body,
        Err(_) => return Vec::new()
    };
    println!("visited {url}");

    let mut new_links = Vec::new();
    let mut seen = visited.lock().await;
    for link in extract_links(&html, &url) {
        if seen.insert(link.clone()) {
            new_links.push(link);
        }
    }
    new_links
}