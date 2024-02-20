use std::{fs, path::Path};

use ai_ollama::{consts::MODEL, Result};

use ollama_rs::Ollama;
use simple_fs::{ensure_dir, read_to_string, save_be_f64, save_json};

const MOCK_DATA_DIR: &str = "_mock-data";
const C04_DIR: &str = "debug-logs/.c04-data";

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    ensure_dir(C04_DIR)?;

    let txt = read_to_string(Path::new(MOCK_DATA_DIR).join("for-embeddings.txt"))?;
    let splits = simple_text_splitter(&txt, 500)?;

    println!("--> Splits: {:?}", splits.len());

    for (i, seg) in splits.into_iter().enumerate() {
        println!();
        // Save as txt
        let file_name = format!("c04-embeddings-{:0>2}.txt", i);
        let path = Path::new(C04_DIR).join(file_name);
        fs::write(path, &seg)?;

        println!("--> Text Length: {}", txt.len());

        let res = ollama
            .generate_embeddings(MODEL.to_string(), seg, None)
            .await?;

        println!("--> Embeddings Length: {:?}", res.embeddings.len());

        // Save as json
        let file_name = format!("c04-embeddings-{:0>2}.json", i);
        save_json(Path::new(C04_DIR).join(file_name), &res.embeddings)?;

        // Save as Binary
        let file_name = format!("c04-embeddings-{:0>2}.be-f64.bin", i);
        let path = Path::new(C04_DIR).join(file_name);
        save_be_f64(&path, &res.embeddings)?;
    }

    Ok(())
}

// * A simple text splitter on "char" number only.
// ! ONLY USED FOR TESTING PURPOSES
fn simple_text_splitter(txt: &str, num: u32) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let mut last = 0;
    let mut count = 0;

    for (i, _) in txt.char_indices() {
        count += 1;
        if count == num {
            result.push(&txt[last..i + 1]);
            last = i + 1;
            count = 0;
        }
    }

    // Handle any remaining text
    if last < txt.len() {
        result.push(&txt[last..]);
    }

    Ok(result.into_iter().map(String::from).collect())
}
