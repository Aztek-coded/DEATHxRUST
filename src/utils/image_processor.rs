use crate::utils::error::BotError;
use image::{DynamicImage, GenericImageView};
use palette::{FromColor, Lab, Srgb};
use std::collections::HashMap;
use std::time::Duration;
use tracing::debug;

pub async fn fetch_avatar(url: &str) -> Result<Vec<u8>, BotError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    debug!("Downloading avatar from: {}", url);

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| BotError::Other(format!("Failed to fetch avatar: {}", e)))?;

    if !response.status().is_success() {
        return Err(BotError::Other(format!(
            "Avatar download failed with status: {}",
            response.status()
        )));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| BotError::Other(format!("Failed to read avatar data: {}", e)))?;

    if bytes.len() > 10 * 1024 * 1024 {
        return Err(BotError::Other(
            "Avatar file too large (max 10MB)".to_string(),
        ));
    }

    Ok(bytes.to_vec())
}

pub fn extract_dominant_color(image_data: &[u8]) -> Result<u32, BotError> {
    let img = image::load_from_memory(image_data)
        .map_err(|e| BotError::Other(format!("Failed to decode image: {}", e)))?;

    let resized = resize_for_processing(&img);

    let dominant = match find_dominant_by_kmeans(&resized, 5) {
        Some(color) => color,
        None => find_dominant_by_histogram(&resized),
    };

    Ok(dominant)
}

pub fn extract_dual_colors(image_data: &[u8]) -> Result<(u32, u32), BotError> {
    let img = image::load_from_memory(image_data)
        .map_err(|e| BotError::Other(format!("Failed to decode image: {}", e)))?;

    let resized = resize_for_processing(&img);

    let dual_colors = match find_dual_colors_by_kmeans(&resized, 5) {
        Some(colors) => colors,
        None => {
            let single_color = find_dominant_by_histogram(&resized);
            (single_color, single_color)
        }
    };

    Ok(dual_colors)
}

fn resize_for_processing(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    if width <= 256 && height <= 256 {
        return img.clone();
    }

    let scale = (256.0 / width.max(height) as f32).min(1.0);
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;


    img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
}

fn find_dominant_by_kmeans(img: &DynamicImage, k: usize) -> Option<u32> {
    let pixels: Vec<Lab> = img
        .pixels()
        .filter_map(|(_, _, rgba)| {
            let rgba = rgba.0;
            if rgba[3] < 128 {
                return None;
            }

            let rgb = Srgb::new(
                rgba[0] as f32 / 255.0,
                rgba[1] as f32 / 255.0,
                rgba[2] as f32 / 255.0,
            );
            Some(Lab::from_color(rgb))
        })
        .collect();

    if pixels.is_empty() {
        return None;
    }

    let mut centroids: Vec<Lab> = pixels
        .iter()
        .step_by(pixels.len() / k)
        .take(k)
        .cloned()
        .collect();

    let mut assignments = vec![0; pixels.len()];

    for _ in 0..20 {
        for (i, pixel) in pixels.iter().enumerate() {
            let mut min_dist = f32::MAX;
            let mut closest = 0;

            for (j, centroid) in centroids.iter().enumerate() {
                let dist = color_distance(pixel, centroid);
                if dist < min_dist {
                    min_dist = dist;
                    closest = j;
                }
            }

            assignments[i] = closest;
        }

        let mut new_centroids = vec![Lab::new(0.0, 0.0, 0.0); k];
        let mut counts = vec![0; k];

        for (i, pixel) in pixels.iter().enumerate() {
            let cluster = assignments[i];
            new_centroids[cluster].l += pixel.l;
            new_centroids[cluster].a += pixel.a;
            new_centroids[cluster].b += pixel.b;
            counts[cluster] += 1;
        }

        for i in 0..k {
            if counts[i] > 0 {
                new_centroids[i].l /= counts[i] as f32;
                new_centroids[i].a /= counts[i] as f32;
                new_centroids[i].b /= counts[i] as f32;
            }
        }

        centroids = new_centroids;
    }

    let mut cluster_sizes = vec![0; k];
    for &assignment in &assignments {
        cluster_sizes[assignment] += 1;
    }

    let largest_cluster = cluster_sizes
        .iter()
        .enumerate()
        .max_by_key(|(_, &size)| size)
        .map(|(idx, _)| idx)?;

    let dominant_lab = centroids[largest_cluster];
    let dominant_rgb: Srgb = Srgb::from_color(dominant_lab);

    let r = (dominant_rgb.red * 255.0) as u8;
    let g = (dominant_rgb.green * 255.0) as u8;
    let b = (dominant_rgb.blue * 255.0) as u8;

    Some(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
}

fn find_dominant_by_histogram(img: &DynamicImage) -> u32 {
    let mut color_counts: HashMap<u32, u32> = HashMap::new();

    for (_, _, rgba) in img.pixels() {
        let rgba = rgba.0;

        if rgba[3] < 128 {
            continue;
        }

        let r = (rgba[0] / 16) * 16;
        let g = (rgba[1] / 16) * 16;
        let b = (rgba[2] / 16) * 16;

        let quantized = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        *color_counts.entry(quantized).or_insert(0) += 1;
    }

    color_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(color, _)| color)
        .unwrap_or(0x5865F2)
}

fn find_dual_colors_by_kmeans(img: &DynamicImage, k: usize) -> Option<(u32, u32)> {
    let pixels: Vec<Lab> = img
        .pixels()
        .filter_map(|(_, _, rgba)| {
            let rgba = rgba.0;
            if rgba[3] < 128 {
                return None;
            }

            let rgb = Srgb::new(
                rgba[0] as f32 / 255.0,
                rgba[1] as f32 / 255.0,
                rgba[2] as f32 / 255.0,
            );
            Some(Lab::from_color(rgb))
        })
        .collect();

    if pixels.is_empty() {
        return None;
    }

    let mut centroids: Vec<Lab> = pixels
        .iter()
        .step_by(pixels.len() / k)
        .take(k)
        .cloned()
        .collect();

    let mut assignments = vec![0; pixels.len()];

    for _ in 0..20 {
        for (i, pixel) in pixels.iter().enumerate() {
            let mut min_dist = f32::MAX;
            let mut closest = 0;

            for (j, centroid) in centroids.iter().enumerate() {
                let dist = color_distance(pixel, centroid);
                if dist < min_dist {
                    min_dist = dist;
                    closest = j;
                }
            }

            assignments[i] = closest;
        }

        let mut new_centroids = vec![Lab::new(0.0, 0.0, 0.0); k];
        let mut counts = vec![0; k];

        for (i, pixel) in pixels.iter().enumerate() {
            let cluster = assignments[i];
            new_centroids[cluster].l += pixel.l;
            new_centroids[cluster].a += pixel.a;
            new_centroids[cluster].b += pixel.b;
            counts[cluster] += 1;
        }

        for i in 0..k {
            if counts[i] > 0 {
                new_centroids[i].l /= counts[i] as f32;
                new_centroids[i].a /= counts[i] as f32;
                new_centroids[i].b /= counts[i] as f32;
            }
        }

        centroids = new_centroids;
    }

    let mut cluster_sizes = vec![0; k];
    for &assignment in &assignments {
        cluster_sizes[assignment] += 1;
    }

    let mut cluster_size_indices: Vec<(usize, usize)> = cluster_sizes
        .iter()
        .enumerate()
        .map(|(idx, &size)| (idx, size))
        .collect();

    cluster_size_indices.sort_by_key(|(_, size)| std::cmp::Reverse(*size));

    if cluster_size_indices.len() < 2 || cluster_size_indices[1].1 == 0 {
        return None;
    }

    let primary_idx = cluster_size_indices[0].0;
    let secondary_idx = cluster_size_indices[1].0;

    let primary_color = convert_lab_to_rgb(centroids[primary_idx]);
    let secondary_color = convert_lab_to_rgb(centroids[secondary_idx]);

    Some((primary_color, secondary_color))
}

fn convert_lab_to_rgb(lab: Lab) -> u32 {
    let rgb: Srgb = Srgb::from_color(lab);
    let r = (rgb.red * 255.0) as u8;
    let g = (rgb.green * 255.0) as u8;
    let b = (rgb.blue * 255.0) as u8;

    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn color_distance(a: &Lab, b: &Lab) -> f32 {
    let dl = a.l - b.l;
    let da = a.a - b.a;
    let db = a.b - b.b;
    (dl * dl + da * da + db * db).sqrt()
}
