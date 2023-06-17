// src/models/vector_search/cosine_similarity.rs
use crate::models::{MEMORY_CACHE, EmbeddingsData, SimilarityResult};

pub fn top_n_similar(vec: Vec<f32>, n: usize) -> Vec<SimilarityResult> {
    let cache = MEMORY_CACHE.read().unwrap();
    println!("Number of JSONs in memory cache: {}", cache.len());
    let mut similarities: Vec<(f32, &EmbeddingsData)> = Vec::new();

    for data in cache.values() {
        let dot_product: f32 = vec.iter().zip(&data.embedding).map(|(a, b)| a * b).sum();
        let vec_magnitude: f32 = vec.iter().map(|v| v.powi(2)).sum::<f32>().sqrt();
        let data_magnitude: f32 = data.embedding.iter().map(|v| v.powi(2)).sum::<f32>().sqrt();
        let similarity: f32 = dot_product / (vec_magnitude * data_magnitude);

        similarities.push((similarity, data));
    }

    // 对结果进行排序，选择前n个最相似的
    similarities.sort_by(|(sim_a, _), (sim_b, _)| sim_b.partial_cmp(sim_a).unwrap());

    let mut results: Vec<SimilarityResult> = Vec::new();
    for (_, data) in similarities.into_iter().take(n) {
        results.push(SimilarityResult {
            input: data.input.clone(),
            uuid: data.uuid.clone(),
        });
    }

    results
}