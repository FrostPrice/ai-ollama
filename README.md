# Rust AI Ollama

AI client interface method for Ollama models. This client is written in Rust and uses the ANY of the AI model.

## Require

- [Rust Stable](https://rustup.rs)
- [Ollama](https://ollama.ai)
- (Optional) A Good GPU ;)

Or using **[Docker](https://www.docker.com/) (Recomended)**

## How to run

### Starting dev environment

- There are 2 ways that this can be achieved:

  - Using `cargo build` and `cargo run` in sequence
  - **(Recomended)** using `cargo watch` to enable live reload of the APP and on development testing. Run the following: `cargo watch -q -c -x "run -q --example <EXAMPLE_NAME>"` for just the live-reload.

- You may execute each example isolated:
  - `cargo run -q --example <EXAMPLE_NAME>`

### Starting AI Model

- To start the AI model, you can use the following command: `docker-compose up` or you may install Ollama CLI locally
- If you want to use the Ollama CLI, you can use the following command: `ollama run <MODEL_NAME>
- If you want to use the Ollama CLI with Docker, you can use the following command: `docker exec -it ollama ollama run <MODEL_NAME>`
- After running the command, make sure you informed the correct Model in the /src/lib.rs MODEL constant
- Also, check bellow for `How to configure Docker to be able to use the GPU`

**Important:** The docker-compose.yaml has not enable the GPU usage by default. Please check the file and make the necessary adjustments.

### Which model to use

- You can use any model from Ollama, just replace the `MODEL_NAME` in the command above with the model you want to use.
- The best model found were:
  - mixtral (26GB)
  - dolphin-mixtral (26GB)

### OBS

Just be mindful that the model you choose will impact the time it takes to run the AI model. The bigger the model, the longer it will take to run. And the bigger the model, the more resources it will consume.

## How to configure Docker to be able to use the GPU (Only for Nvidia, for now)

- First, you need to install the Nvidia Container Toolkit. This will vary depending on your OS. Most OS's have the package named nvidia-container-toolkit to install it.
  - Apt: `sudo apt-get install nvidia-container-toolkit`
  - Dnf: `sudo dnf install nvidia-container-toolkit`
  - Arch: See the [AUR](https://aur.archlinux.org/packages/nvidia-container-toolkit)
- After installing the nvidia-container-toolkit, you need to tell docker to use the Nvidia Runtime
  - sudo nvidia-ctk runtime configure --runtime=docker
- Finally, you need to restart the docker service
  - sudo systemctl restart docker
- Now you can use the GPU with Docker
