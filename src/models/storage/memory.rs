// src/models/storage/memory.rs
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::RwLock;
use glob::glob;
use lazy_static::lazy_static;
use crate::models::EmbeddingsData;
// use rayon::prelude::*;

lazy_static! {
    pub static ref MEMORY_CACHE: RwLock<HashMap<String, EmbeddingsData>> = {
        let mut cache = HashMap::new();
        match load_all_embeddings("wiki_5") {
            Ok(data) => cache = data,
            Err(e) => eprintln!("Error loading embeddings data: {}", e),
        };
        println!("cache.len(): {:?}", cache.len());
        RwLock::new(cache)
    };
}

pub fn load_all_embeddings(dir: &str) -> Result<HashMap<String, EmbeddingsData>, Box<dyn std::error::Error>> {
    let mut embeddings = HashMap::new();

    for entry in glob(&format!("{}/*.json", dir))? {
        match entry {
            Ok(path) => {
                let mut file = File::open(path)?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                if let Ok(data) = serde_json::from_str::<EmbeddingsData>(&contents) {
                    embeddings.insert(data.uuid.clone(), data);
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }

    Ok(embeddings)
}


// // 查询函数使用缓存
// fn search_with_cache(uuid: &str) -> Option<EmbeddingsData> {
//     let cache = MEMORY_CACHE.read().unwrap();
//     cache.get(uuid).cloned()
// }

// // 查询函数不使用缓存，每次都会重新从文件中加载
// fn search_without_cache(uuid: &str, dir: &str) -> Option<EmbeddingsData> {
//     match load_all_embeddings(dir) {
//         Ok(embeddings) => embeddings.get(uuid).cloned(),
//         Err(_) => None,
//     }
// }

// // 计算平方根的函数使用缓存
// fn calculate_sqrt_sum(data: &EmbeddingsData) -> f32 {
//     data.embedding.iter().map(|v| v.powi(2)).sum::<f32>().sqrt()
// }


// #[cfg(test)]
// mod tests {
//     use super::*;
//     //use std::ops::Deref;
//     use std::time::Instant;

//     // // 测试使用缓存的查询函数和不使用缓存的查询函数
//     // #[test]
//     // fn test_search_speed() {
//     //     let uuid_to_check = "1cccd5d1-97e2-4dda-a983-6ef639009a6e.json".to_string();
//     //     let dir = "data.json/wiki_10000"; // 替换为你的实际目录

//     //     // 测量使用缓存的查询时间
//     //     let start = Instant::now();
//     //     let _result = search_with_cache(&uuid_to_check);
//     //     let duration_with_cache = start.elapsed();

//     //     // 测量不使用缓存的查询时间
//     //     let start = Instant::now();
//     //     let _result = search_without_cache(&uuid_to_check, dir);
//     //     let duration_without_cache = start.elapsed();

//     //     println!("Time taken with cache: {:?}", duration_with_cache);
//     //     println!("Time taken without cache: {:?}", duration_without_cache);
        
//     //     // 确保使用缓存的查询时间小于不使用缓存的查询时间
//     //     assert!(duration_with_cache < duration_without_cache);
//     // }

// #[test]
// fn test_sqrt_speed() {
//     let dir = "data.json/wiki_10000"; // 替换为你的实际目录

//     let cache = MEMORY_CACHE.read().unwrap();
//     // 计算使用缓存时的计算平方根时间
//     let start = Instant::now();
//     let data_values: Vec<&EmbeddingsData> = cache.values().collect();
//     let _sqrt_sums: Vec<f32> = data_values.par_iter().map(|&data| calculate_sqrt_sum(data)).collect();
//     let duration_with_cache = start.elapsed();
//     drop(cache); // 释放锁，以便接下来可以重新加载所有嵌入

//     // 计算不使用缓存时的计算平方根时间
//     let start = Instant::now();
//     match load_all_embeddings(dir) {
//         Ok(embeddings) => {
//             let data_values: Vec<&EmbeddingsData> = embeddings.values().collect();
//             let _sqrt_sums: Vec<f32> = data_values.par_iter().map(|&data| calculate_sqrt_sum(data)).collect();
//         }
//         Err(e) => eprintln!("Error loading embeddings data: {}", e),
//     };
//     let duration_without_cache = start.elapsed();

//     println!("Time taken with cache: {:?}", duration_with_cache);
//     println!("Time taken without cache: {:?}", duration_without_cache);
    
//     // 确保使用缓存的时间小于不使用缓存的时间
//     assert!(duration_with_cache < duration_without_cache);
// }

// }
