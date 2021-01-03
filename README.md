# guild

Guild is a Command Line Interface build with Rust that posts code snippets to a Discord channel using Webhooks.

# How does it work?

Guild works by utilizing [Serenity](https://www.github.com/serenity-rs/serenity) to access the Discord API. In order to communicate with a specified Discord text channel, Guild uses Discord Webhooks to send the code snippets from a specified file to the channel.

# Download

## Install Rust

In order to download guild, you need to have Rust downloaded on your computer if you don't already. To install Rust, use one of the following commands based on your computer's Operating System:

#### Unix 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
#### Windows
Download the following executable and follow the setup instructions: [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)


If the downloads go well, you will see the following message: 
```
Rust is installed now. Great!
```

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
    guid post <file path> <starting line> <ending line>
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
