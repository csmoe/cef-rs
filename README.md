
<div align="center">
  <picture>
    <img src="cef-rs.png" alt="cef-rs logo" width="20%"/>
  </picture>

  <p>cef-rs - Emebbed CEF into Rust Application</p>
</div>

## Supported Targets

| Target | Linux | macOS | Windows |
| ------ | ----- | ----- | ------- |
| x86_64 | ✅    | ✅   | ✅      |
| ARM64  | ✅    | ✅   | ✅      |

## Usage

### Linux

#### Manual Install

- Download Linux-64bit prebuilt with `sys/upgrade.rs`:

```sh
cd sys
./upgrade.rs x86_64-unknown-linux-gnu --download
```

- Build and run the application with `LD_LIBRARY_PATH` (or you can also add rpath to your cargo config or build script):

```sh
LD_LIBRARY_PATH=sys/cef/archive cargo r --example demo
```

#### Flatpak

- Install flatpak runtime & sdk:

```
flatpak install flathub dev.crabnebula.Platform
flatpak install flathub dev.crabnebula.Sdk
```

- Setup cargo project for flatpak. See [flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools/blob/master/cargo/README.md) for more details. Here are files you will need to have at leaset:
  - flatpak-cargo-generator.py
  - flatpak manifest file (ie. app.example.demo.yml)

- Build the flatpak application and run:

```
cargo b --example demo
python3 ./flatpak-cargo-generator.py ./Cargo.lock -o cargo-sources.json
touch run.sh
flatpak-builder --user --install --force-clean target app.example.demo.yml
flatpak run app.example.demo
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

