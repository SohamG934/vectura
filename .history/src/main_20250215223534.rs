mod chunk;
mod embed;
mod hnsw;
mod create_tokenizer;

use std::io;
use chunk::{read_text_from_file, chunk_text};
use embed::{embed_chunks, save_embeddings, load_embeddings};
use hnsw::HNSW;
use std::path::Path;

fn main() {
    let filename = "story.txt";
    let tokenizer_path = "tokenizer.json";
    let embeddings_file = "embeddings.bin";
    let chunk_size = 50;
    let overlap = 10;

    // Step 0: Load tokenizer, create if not exists
    if !Path::new(tokenizer_path).exists() {
        create_tokenizer::create().expect("Failed to create tokenizer");
        println!("Tokenizer created successfully!");
    }
    // Step 1: Read text and chunk it
    let text = read_text_from_file(filename);
    let chunks = chunk_text(&text, chunk_size, overlap);

    // Step 2: Embed chunks
    let embeddings = embed_chunks(chunks, tokenizer_path);

    // Step 3: Save embeddings to disk
    save_embeddings(embeddings_file, &embeddings);

    // Step 4: Load embeddings from disk
    let loaded_embeddings = load_embeddings(embeddings_file);

    // Step 5: Build HNSW index from loaded embeddings
    let hnsw = HNSW::from_embeddings(loaded_embeddings.clone());

    // Step 6: Accept User Query
    println!("Enter your query:");
    let mut query = String::new();
    io::stdin().read_line(&mut query).expect("Failed to read input");

    // Step 7: Embed Query
    let query_embedding = embed_chunks(vec![query.clone()], tokenizer_path);

    // Step 8: Search for the best match
    if let Some(best_response) = hnsw.search(&query_embedding[0].vector) {
        println!("Best response: {}", best_response);
    } else {
        println!("No relevant document found.");
    }
}
