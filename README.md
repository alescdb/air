# AI in Rust

A simple rust program to call ChatGPT API from command line with markdown to ANSI output.
This project replaces `aid` (written in Dart) and implement `chatgpt` and `llama-cpp`.

### Note

This project is an excuse for me to discover Rust, it might (certainly) need optimizations.

### Install

On ArchLinux :
```bash
cd archlinux
makepkg -si
```

or
```bash
make install
```

Other :

```bash
make install-home
```
Note: this will install `air` in `$HOME/.cargo/bin` by default

### Usage

```
Usage: air [options] <prompt>

Options:
    -l, --local name    Run local model (llama-cpp)
    -c, --clear         Clear history
    -v, --verbose       Verbose/debug
    -m, --markdown      Display as markdown
    -h, --help          Help
```

### Setup

```json
{
  "apikey": "<openai api key>",
  "model": "gpt-4-1106-preview",
  "system": "You are a Linux coder assistant.",
  "markdown": true,
  "expiration": 86400,
  "main_gpu": "",
  "local": [
    {
      "name": "codellama",
      "model": "/opt/models/codellama-7b.Q5_K_M.gguf",
      "prompt": null
    },
    {
      "name": "vigogne",
      "model": "/opt/models/vigogne-2-7b-chat.Q4_K_M.gguf",
      "prompt": "{system}\n\n<|UTILISATEUR|>: {prompt}\n<|ASSISTANT|>: \n"
    }
  ]
}
```

| name         | value                              |
|--------------|------------------------------------|
| `apikey`     | OpenAI API key (required)          |
| `model`      | Set by default to `gpt-4`          |
| `markdown`   | Parse markdown (default to `true`) |
| `system`     | System prompt (not required)       |
| `expiration` | Hisory expiration (in sec)         |
| `main_gpu`   | See [llama_cpp_rs](https://crates.io/crates/llama_cpp_rs/) |

Local llama models :

| name         | value                              |
|--------------|------------------------------------|
| `name`       | Name (use with `-l`)               |
| `model`      | Model path                         |
| `prompt`     | Prompt format for the model        |

### TODO

- [x] Llama cpp support
- [ ] Publish ArchLinux AUR
- [ ] Any idea ?
