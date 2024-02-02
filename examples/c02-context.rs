use ai_ollama::consts::MODEL;
use ai_ollama::gen::gen_stream_print;
use ai_ollama::Result;

use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationContext;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Ollama API Example: Context (c02-context.rs)\n");

    // By default access localhost:11434
    let ollama = Ollama::default();

    let prompts = &[
        "What is the result of 1+1? (Be concise, I just need the result and no more explanations)",
        "Sorry, what was my first question?",
    ];

    let mut last_context: Option<GenerationContext> = None;

    for prompt in prompts {
        println!("--> Prompt: {}", prompt);
        let mut gen_request = GenerationRequest::new(MODEL.to_string(), prompt.to_string());

        if let Some(last_context) = last_context.take() {
            gen_request = gen_request.context(last_context);
        }

        let final_data = gen_stream_print(&ollama, gen_request).await?;

        if let Some(final_data) = final_data {
            last_context = Some(final_data.context);

            // Save for Debug
            let context_file_path = "debug-logs/context.json";
            // TODO!: Remove the simple-fs crate and use the built-in std::fs
            simple_fs::ensure_file_dir(context_file_path)?;
            simple_fs::save_json(context_file_path, &last_context)?;
        }
    }

    Ok(())
}
