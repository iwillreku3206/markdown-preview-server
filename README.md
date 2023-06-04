# Markdown Preview Server

A 🔥blazingly fast🔥 markdown preview server written in Rust

## Project Goals

* **Easy to extend**\
Under the hood, the server uses the Rust port of the `markdown-it` library, which is made in a way where extensions to the base CommonMark spec can easily be created
* **Fast**\
Each render of the document should ideally be lower than 16.6ms, which is the time that a frame is shown on a 60Hz monitor.

## Features

* **CommonMark compliance**. The server uses the [`markdown-it.rs`](https://crates.io/crates/markdown-it) library, which has full CommonMark compliance. Some additional features/blocks of the server may, however, replace some CommonMark blocks with versions that are not 100% compliant. There are plans to add a setting to disable these features
* **LaTeX support**. The server is able to translate LaTeX functions into MathML for rendering in the browser
* **Syntax highlighting**. The server is able to highlight code blocks using the [`syntect`](https://crates.io/crates/syntect) library
* **Table of Contents**. The server is able to generate a table of contents for the document using the `[toc]` block
* **Custom CSS**. The server can be configured to use a custom CSS file for styling the preview page

## Installation

### From Source

In order to build the server, run the following commands:

```bash
 # First, we build the front-end preview page
cd src_web
yarn
yarn build 
mv build ../assets/web_build

 # Then, compile the server
cd ..
cargo build --release
```

The final build will be in `target/release/markdown-preview-server`

## Usage

`markdown-preview-server` is intended to run with an editor plugin. Currently, there is only a [Neovim plugin](https://github.com/iwillreku3206/markdown-preview-server) currently available, but there are plans to make a VSCode/Codium plugin

```
$ markdown-preview-server --help
Usage: markdown-preview-server [OPTIONS]

Options:
      --css <PATH>             Path to user-defined CSS file [default: /etc/markdown-preview-server/style.default.css]
  -p, --port <PORT>            Port to listen on [default: 8080]
      --websocket-port <PORT>  [default: 8081]
  -h, --help                   Print help
  -V, --version                Print version
```
