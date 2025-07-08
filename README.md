
# Git Time Metric (GTM) Plugin for KiCad

**kicad-gtm** is a plugin that integrates [KiCad](https://www.kicad.org/) with [Git Time Metric (GTM)](https://github.com/git-time-metric/gtm). It allows you to track time spent on your KiCad projects locally, with data stored directly in your git repository's notes.

## Disclaimer
This plugin is currently under development. While functional, it should be considered experimental. Please back up your project data, especially your git notes, before extensive use. Proceed at your own risk.

## Installation

On all platforms:


1.  **Install GTM CLI:**
    Follow the installation instructions for the GTM command-line interface from the official GTM repository: [https://github.com/git-time-metric/gtm](https://github.com/git-time-metric/gtm). Ensure `gtm` is correctly installed and accessible in your system's PATH.

2.  **Download `kicad-gtm` Plugin:**
    Download the latest release of `kicad-gtm` (as a ZIP archive, e.g., `kicad-gtm-plugin.zip`) from this repository's releases section.

3.  **Install in KiCad:**
    Open KiCad, go to "Tools" -> "Plugin and Content Manager". Click "Install from File..." and select the downloaded `kicad-gtm-plugin.zip` file.

4.  **Configure `kicad-gtm`:**
    After installation, the `kicad-gtm` plugin application might launch automatically, or you might need to find it in your applications menu (this behavior is platform-dependent). Open the `kicad-gtm` application. You will need to configure the path to your KiCad projects folder.

5.  **Start Designing!**
    `kicad-gtm` will monitor your KiCad activity in the specified projects folder and record time using the GTM CLI.

## Configuration

*   **Projects Folder:** The primary setting in `kicad-gtm` is the "Projects Folder". This should be set to the directory where you store your KiCad projects. Each KiCad project you want to track must be within a Git repository.

*   **Initialize GTM for Projects:** For each KiCad project (that is a Git repository) you want to track, you need to initialize GTM. Navigate to the project's root directory in your terminal and run:
    ```shell
    gtm init
    ```

*   **No API Keys Needed:** Unlike plugins for cloud-based services, `kicad-gtm` works locally with your GTM CLI and Git repositories. It does not require any API keys or external service URLs.

## Building from Source

If you prefer to build `kicad-gtm` from the main branch:

1.  **Install Rust and Cargo:** Ensure you have a recent Rust toolchain installed. The recommended way is via [rustup](https://rustup.rs/). If you have rustup, you can set the default toolchain:
    ```shell
    rustup default stable
    ```

2.  **Clone Repository:** Clone this repository:
    ```shell
    git clone https://github.com/LokiMetaSmith/kicad-gtm.git
    cd kicad-gtm
    ```

3.  **Build the Plugin:** The Rust project for the plugin is currently in the `kicad-wakatime` directory (this may be renamed to `kicad-gtm` in the future). Navigate into it and build:
    ```shell
    cd kicad-wakatime
    cargo build --release
    ```
    The optimized executable will be located at `target/release/kicad-gtm` (or `target/release/kicad-gtm.exe` on Windows). Note: The actual executable name inside `target/release/` might be `kicad-wakatime` or `kicad-wakatime.exe` if the `name` field in `Cargo.toml` has not been updated to `kicad-gtm` prior to building.

4.  **System Dependencies:** CMake is not a direct requirement for building the Rust plugin itself. However, ensure any system dependencies for the libraries used by the Rust project (e.g., for GUI, file dialogs like `rfd`) are installed. These typically include development packages for X11/Wayland on Linux.

The code in the main branch should be considered unstable, as some features may still be in progress between releases.

## Creating the Plugin ZIP Archive

For manual installation via KiCad's Plugin and Content Manager ("Install from File..."), or for distributing the plugin, a ZIP archive with a specific structure is needed. KiCad expects `metadata.json` to be at the root of the archive.

The required structure for the ZIP file (e.g., `kicad-gtm-plugin.zip`) is:
```
kicad-gtm-plugin.zip/
├── kicad-gtm          # The compiled executable (or kicad-gtm.exe on Windows)
├── metadata.json      # Plugin metadata file
└── resources/         # Optional directory for icons
    └── icon.png       # Optional icon (e.g., 64x64 PNG)
```

**Example script to create the archive (Linux/macOS):**

```shell
# 1. Define project paths (adjust if necessary)
PLUGIN_BUILD_DIR="kicad-wakatime/target/release"
# Check executable name, it might be kicad-wakatime if Cargo.toml name isn't updated yet
EXECUTABLE_NAME="kicad-gtm" # or "kicad-wakatime"
METADATA_FILE="kicad-wakatime/metadata.json"
OUTPUT_ZIP="kicad-gtm-plugin.zip"
STAGING_DIR="kicad-gtm-package-staging"

# Ensure you have built the plugin first (see "Building from Source")
# Example: cd kicad-wakatime && cargo build --release && cd ..

# 2. Prepare files in a staging directory
echo "Preparing files for packaging..."
mkdir -p "${STAGING_DIR}/resources"

if [ -f "${PLUGIN_BUILD_DIR}/${EXECUTABLE_NAME}" ]; then
    cp "${PLUGIN_BUILD_DIR}/${EXECUTABLE_NAME}" "${STAGING_DIR}/"
elif [ -f "${PLUGIN_BUILD_DIR}/kicad-wakatime" ]; then # Fallback if name not updated
    echo "Warning: Using 'kicad-wakatime' as executable name."
    cp "${PLUGIN_BUILD_DIR}/kicad-wakatime" "${STAGING_DIR}/${EXECUTABLE_NAME}"
else
    echo "Error: Executable not found in ${PLUGIN_BUILD_DIR} with name ${EXECUTABLE_NAME} or kicad-wakatime."
    exit 1
fi

cp "${METADATA_FILE}" "${STAGING_DIR}/"
# cp path/to/your/icon.png "${STAGING_DIR}/resources/" # Optional: copy your icon

# 3. Create the ZIP archive from within the staging directory
echo "Creating ZIP archive..."
cd "${STAGING_DIR}"
zip -r "../${OUTPUT_ZIP}" .
cd ..

# 4. Cleanup staging directory (optional)
# rm -rf "${STAGING_DIR}"

echo "Created ${OUTPUT_ZIP} successfully."
```

For Windows, you would replace `kicad-gtm` with `kicad-gtm.exe` (or `kicad-wakatime.exe`) in the `cp` command and use a Windows-compatible zipping tool (e.g., 7-Zip, or built-in "Send to > Compressed (zipped) folder" functionality after preparing files in a folder).

## Note on KiCad Versions
The KiCad version compatibility notes from the original WakaTime plugin may still apply if the underlying project structure interaction remains similar. For now, KiCad 8.0.7 stable is recommended. Using nightly versions (like 8.99) might lead to project files that cannot be opened by older KiCad versions.

<details>
<summary>Downloading KiCAD Nightly (If needed for other reasons)</summary>


If you are a Windows user, you can download KiCAD 9 [here](https://downloads.kicad.org/kicad/windows/) (pick an "x86_64.exe".)

If you are a macOS user, you can download KiCAD 8.99 [here](https://downloads.kicad.org/kicad/macos/) (pick a ".dmg").

If you are an Ubuntu user, you can install KiCAD 9 using the following shell commands:

```shell
sudo add-apt-repository --yes ppa:kicad/kicad-9.0-releases
sudo apt update
sudo apt install --install-recommends kicad
```

</details>

## Issues

If `kicad-gtm` is not doing what you expect, please [open an issue](https://github.com/LokiMetaSmith/kicad-gtm/issues) on this repository's issue tracker. Please provide details about your operating system, KiCad version, `kicad-gtm` plugin version, and steps to reproduce the issue.
=======
If kicad-gtm is not doing what you expect, please [open an issue](https://github.com/LokiMetaSmith/kicad-gtm/issues).

The bug report template will ask you for a magic word to confirm that you've read this README.\
The magic word is **"dreadnought"**.

