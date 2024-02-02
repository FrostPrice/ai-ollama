use ai_ollama::consts::{DEFAULT_SYSTEM_MOCK, MODEL};
use ai_ollama::gen::gen_stream_print;
use ai_ollama::Result;

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Ollama API Example: Simple (c01-simple.rs)\n");

    // By default access localhost:11434
    let ollama = Ollama::default();

    let prompt = "What is the best hamburger in the world?".to_string();

    let gen_request =
        GenerationRequest::new(MODEL.to_string(), prompt).system(DEFAULT_SYSTEM_MOCK.to_string());

    // -- Single Response Generation
    // let gen_response = ollama.generate(gen_request).await?;
    // println!("Response: {}", gen_response.response);

    // -- Stream Response Generation
    gen_stream_print(&ollama, gen_request).await?;

    Ok(())
}
