# chgpf

It is a thin wrapper around Git config, a simple command-line tool to change Git profiles with ease. Chgpf changes your global Git config by the settings you configured. It helps you with switching between several git accounts you may have.

![showcase](docs/assets/chgpf_showcase.gif)

## Table of Contents

- [Short Explanation](<README#Short Explanation>)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

## Short Explanation

First of all, you define a config with profile aliases. You can define fields to be changed by `chgpf`. Those aliases are used to switch between Git configurations.

`chgpf` DOES NOT persist your Git configuration state. But it also does nothing critical or dangerous. It only helps you switching between profiles in a single command.

## Installation

To install `chgpf`, you can check *Releases* page here or just install with `cargo`:

```bash
cargo install chgpf --git https://github.com/kulichkoff/chgpf
```

## Usage

You can run `chgpf` to switch between Git profiles

```bash
# Syntax
chgpf <profile_name>
```

```bash
# Examples
chgpf home
chgpf work
```

## Configuration

Create a configuration file in your home directory to define your Git
profiles. For Linux and macOS it is `~/.config/chgpf/profiles`.

```toml
[profile_name]
user = "your_email@example.com"
name = "Your Name"
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.
