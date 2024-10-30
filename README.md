# chgpf

A simple command-line tool to change Git profiles with ease.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

## Features

- Switch between multiple Git profiles in a single command.
- Easy configuration and setup.
- Lightweight and fast.

## Installation

To install `chgpf`, you can use *Releases* page or clone the repo and then
install with `cargo`:

```bash
cargo install chgpf
```

## Usage

```bash
chgpf <profile_name>
```

Replace `<profile_name>` with the name of the profile you want to switch to.

## Configuration

Create a configuration file in your home directory to define your Git
profiles. In Unix systems it should be `~/.config/chgpf/profiles`.

```toml
[profile_name]
user = "your_email@example.com"
name = "Your Name"
```

## License

This project is licensed under the MIT License - see the `LICENSE`file
for details.
