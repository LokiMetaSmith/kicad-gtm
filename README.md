# kicad-gtm

**kicad-gtm** is a GTM plugin for [KiCAD](https://www.kicad.org/).
It's based on [kicad-wakatime](https://github.com/hackclub/kicad-wakatime/issues).


## Disclaimer
As of June 2025, **this plugin is likely not suitable for accurate time tracking**.\
It's highly experimental and is not ready for produciton
**Proceed at your own risk.**

## Installation

On all platforms:
1. Download the latest release of kicad-gtm from the releases section. [Click here for downloads.](https://github.com/LokiMetaSmith/kicad-gtm/releases)
2. Open kicad-gtm and fill out the settings.
3. Start designing!

If you know what you're doing, you can build kicad-gtm from the main branch instead. This requires an up-to-date version of [CMake](https://cmake.org) and [protoc](https://grpc.io/docs/protoc-installation).\
The code in the main branch should be considered unstable, as some features may still be in progress between releases.

## Note


<details>
<summary>Downloading KiCAD 9</summary>

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

If kicad-gtm is not doing what you expect, please [open an issue](https://github.com/LokiMetaSmith/kicad-gtm/issues).

The bug report template will ask you for a magic word to confirm that you've read this README.\
The magic word is **"dreadnought"**.
