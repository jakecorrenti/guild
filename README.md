# guild

Guild is a Command Line Interface build with Rust that posts code snippets to a Discord channel using Webhooks.

# How does it work?

Guild works by utilizing [Serenity](https://www.github.com/serenity-rs/serenity) to access the Discord API. In order to communicate with a specified Discord text channel, Guild uses Discord Webhooks to send the code snippets from a specified file to the channel.

# Download

In order to download guild, you need to have Rust downloaded on your computer. To install Rust, use one of the following commands based on your computer's Operating System:

### Unix 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Windows
Download the following executable and follow the setup instructions: [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)


Once Rust is installed on your machine, you must install guild through Cargo: 
```bash
cargo install guild
```

# Usage

How to use
- commands
- setup discord webhook
