# guild

Guild is a Command Line Utility that allows you to post code snippets from a file to a Discord channel.

# How does it work?

By utilizing [Serentiy](https://www.github.com/serenity-rs/serenity) to access the Discord API, guild communicates with your Discord server using Webhooks. This allows guild to post a message to Discord on its own without having to type anything into Discord itself.

# Download

## Install Rust

In order to download guild, you need to have Rust downloaded on your computer if you don't already. To install Rust, go to the [Rust website](https://www.rust-lang.org/tools/install) and follow the instructions.

## Install guild

Once Rust is installed on your machine, you must install guild through Cargo: 
```bash
cargo install guild
```

# Usage

1. Create a [Discord Webhook](https://support.discord.com/hc/en-us/articles/228383668-Intro-to-Webhooks) in the desired server
2. Copy the Discord Webhook URL and execute the following command in your terminal
    ```bash
    guild set <webhook url>
    ```
3. Post your desired code snippet to the Discord server: 
    ```bash
    guild post <file path> <starting line> <ending line>
    ```
    - If you would like the code snippet to have syntax highlighting, add the following flag at the end of your command:
        ```bash
        guild post <file path> <starting line> <ending line> -H
        ```
        or 
        ```bash
        guild post <file path> <starting line> <ending line> --highlight
        ```
    - Example:
        ```bash
        guild post main.c 1 5 -H
        ```
## Commands
- `set`: Set the Discord Webhook URL
- `post`: Post a code snippet to the set Discord channel
## Flags
- `help`: See the possible commands and flags that are supported by guild
- `highlight`: Enable syntax highlighting for the code snippet
