use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        },
        Err(e) => {
            let error_details = format!("Request failed: {}", e);
            ApiResult::NetworkError(error_details)
        },
    }
}

// Try to guess a file extension from the image URL. Falls back to "jpg".
fn guess_extension(url: &str) -> String {
    let trimmed = match url.find('?') {
        Some(idx) => &url[..idx],
        None => url,
    };
    if let Some(dot) = trimmed.rfind('.') {
        let ext = &trimmed[dot + 1..];
        // basic sanity: limit length and chars
        if ext.len() >= 2 && ext.len() <= 5 && ext.chars().all(|c| c.is_ascii_alphanumeric()) {
            return ext.to_lowercase();
        }
    }
    "jpg".to_string()
}

// Download an image at `url` and save it to `dest_path`.
// Returns Err(String) with a human-friendly message on failure.
fn download_image(url: &str, dest_path: &Path) -> Result<(), String> {
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                let mut reader = response.into_reader();
                match File::create(dest_path) {
                    Ok(mut out) => {
                        if let Err(e) = copy(&mut reader, &mut out) {
                            return Err(format!("Failed to write file: {}", e));
                        }
                        Ok(())
                    }
                    Err(e) => Err(format!("Failed to create file {}: {}", dest_path.display(), e)),
                }
            } else {
                Err(format!("Image HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Request failed: {}", e)),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    // Ensure output directory exists
    let images_dir = Path::new("images");
    if let Err(e) = std::fs::create_dir_all(images_dir) {
        eprintln!("Failed to create images directory: {}", e);
        // continue; still attempt fetching images but saving will fail
    }

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);
                // attempt to download the image
                let ext = guess_extension(&dog_image.message);
                let filename = format!("dog_{}.{}", i, ext);
                let dest = images_dir.join(filename);
                match download_image(&dog_image.message, &dest) {
                    Ok(()) => println!("⬇️  Saved to: {}", dest.display()),
                    Err(e) => println!("⚠️  Failed to download image: {}", e),
                }
            },
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
        println!();
    }

    Ok(())
}
