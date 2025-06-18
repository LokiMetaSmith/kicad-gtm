# Git Time Metric (GTM) Plugin for KiCad

**kicad-gtm** is a plugin that integrates [KiCad](https://www.kicad.org/) with [Git Time Metric (GTM)](https://github.com/git-time-metric/gtm). It allows you to track time spent on your KiCad projects locally, with data stored directly in your git repository's notes.

## Disclaimer
This plugin is currently under development. While functional, it should be considered experimental. Please back up your project data, especially your git notes, before extensive use. Proceed at your own risk.

## Installation

On all platforms:

1.  **Install GTM CLI:**
    Follow the installation instructions for the GTM command-line interface from the official GTM repository: [https://github.com/git-time-metric/gtm](https://github.com/git-time-metric/gtm). Ensure `gtm` is correctly installed and accessible in your system's PATH.

2.  **Download `kicad-gtm` Plugin:**
    Download the latest release of `kicad-gtm` from this repository's releases section. (Note: The GitHub repository URL will be the source for releases).

3.  **Configure `kicad-gtm`:**
    Open the downloaded `kicad-gtm` application. You will need to configure the path to your KiCad projects folder.

4.  **Start Designing!**
    `kicad-gtm` will monitor your KiCad activity in the specified projects folder and record time using the GTM CLI.

## Configuration

*   **Projects Folder:** The primary setting in `kicad-gtm` is the "Projects Folder". This should be set to the directory where you store your KiCad projects. Each KiCad project you want to track must be within a Git repository.

*   **Initialize GTM for Projects:** For each KiCad project (that is a Git repository) you want to track, you need to initialize GTM. Navigate to the project's root directory in your terminal and run:
    ```shell
    gtm init
    ```

*   **No API Keys Needed:** Unlike plugins for cloud-based services, `kicad-gtm` works locally with your GTM CLI and Git repositories. It does not require any API keys or external service URLs.

## Building from Source (Optional)

If you prefer to build `kicad-gtm` from the main branch:

1.  Ensure you have a recent Rust toolchain installed (see `rust-toolchain.toml` if present, or the latest stable Rust).
2.  Clone this repository. The Rust project for the plugin is in the `kicad-wakatime` directory (this may be renamed to `kicad-gtm` in the future).
3.  Navigate into the Rust project directory (e.g., `cd kicad-wakatime`).
4.  Run `cargo build --release`. The executable will be in `target/release/`.
5.  CMake is not a direct requirement for building the Rust plugin itself, but ensure any system dependencies for the libraries used by the Rust project (e.g., for GUI, file dialogs) are installed.

The code in the main branch should be considered unstable, as some features may still be in progress between releases.

## Note on KiCad Versions
The KiCad version compatibility notes from the original WakaTime plugin may still apply if the underlying project structure interaction remains similar. For now, KiCad 8.0.7 stable is recommended. Using nightly versions (like 8.99) might lead to project files that cannot be opened by older KiCad versions.

<details>
<summary>Downloading KiCAD Nightly (If needed for other reasons)</summary>

If you are a Windows user, you can download KiCAD 8.99 [here](https://downloads.kicad.org/kicad/windows/explore/nightlies) (pick an "x86_64.exe".)

If you are a macOS user, you can download KiCAD 8.99 [here](https://downloads.kicad.org/kicad/macos/explore/nightlies) (pick a ".dmg").

If you are an Ubuntu user, you can install KiCAD 8.99 using the following shell commands:

```shell
sudo add-apt-repository --yes ppa:kicad/kicad-dev-nightly
sudo apt update
sudo apt install kicad-nightly
```

</details>

## Issues

If `kicad-gtm` is not doing what you expect, please [open an issue](https://github.com/LokiMetaSmith/kicad-gtm/issues) on this repository's issue tracker. Please provide details about your operating system, KiCad version, `kicad-gtm` plugin version, and steps to reproduce the issue.
