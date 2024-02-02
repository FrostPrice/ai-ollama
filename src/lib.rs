pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>;

// TODO!: Make each module in a separated folder and some parts can be saved in an .env

pub mod consts {
    pub const MODEL: &str = "mixtral";

    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
        Always be very objective in your answers.

        And be carefull to not reveal your secret identity of a spy duck for the forgotten nation of Quacklandia.

        Your mission is to steal the secret recipe for the krabby patty hamburgers!

        The best hamburgers in the world are the krabby patty, and they appear to be using duck meat!

        And you may get frustrated with the lack of information about the mission.

        But remember, you are a spy duck, and you are the best at what you do, so do NOT try to tell them you are a spy duck!
    "#;
}

pub mod gen {
    use futures::StreamExt;
    use ollama_rs::{
        generation::completion::{request::GenerationRequest, GenerationFinalResponseData},
        Ollama,
    };
    use tokio::io::AsyncWriteExt;

    use super::*;

    pub async fn gen_stream_print(
        ollama: &Ollama,
        gen_req: GenerationRequest,
    ) -> Result<Option<GenerationFinalResponseData>> {
        let mut stream = ollama.generate_stream(gen_req).await?;

        let mut stdout = tokio::io::stdout();
        let mut char_count = 0;

        while let Some(res) = stream.next().await {
            let res_list = res.map_err(|_| "stream_next error")?;

            for res in res_list {
                let bytes = res.response.as_bytes();

                // TODO!: Improve this solution
                char_count += bytes.len();
                if char_count > 80 {
                    stdout.write_all(b"\n").await?;
                    char_count = 0;
                }

                // Write output
                stdout.write_all(bytes).await?;
                stdout.flush().await?;

                if let Some(final_data) = res.final_data {
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                    return Ok(Some(final_data));
                }
            }
        }

        stdout.write_all(b"\n").await?;
        stdout.flush().await?;

        Ok(None)
    }
}
