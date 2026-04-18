# Steam Uploader Rust
Steam Uploader Rust is the successor to [Steam Uploader](https://github.com/SirDoggyJvla/Steam-Uploader), rewritten in Rust. It uses the binding library [steamworks-rs](https://github.com/Noxime/steamworks-rs) to interact with the Steamworks SDK.

## Usage
The tool is a command-line application that can be used to upload and delete items on the Steam Workshop. It uses a manifest file to store the necessary information for uploading items, such as the app ID, workshop ID, title, description and visibility.

The manifest file needs to be either a JSON, YAML or TOML file and should be named `mod-manifest` (for example `mod-manifest.json`). The tool will automatically look for the manifest file in the current directory but it can also be specified with the `--manifest` flag.

To run the tool, simply execute the following command in the terminal, in the folder where the manifest file is located:
```bash
SteamUploader upload
```

And by manunally specifying the manifest file:
```bash
SteamUploader upload --manifest path/to/manifest.json
```

## Example manifest file
Example manifest files can be found in [the examples folder](test/example_mod/). Here is an example of a manifest file in JSON format:
```json
{
    "appid": 108600,
    "workshopid": 123456789,
    "content": "./Contents",
    "preview": "./preview.png",
    "title": "Example Mod",
    "description": "./description.bbcode",
    "visibility": 2
}
```

## Build
Building is done with Cargo. You can build the project with the following command:
```bash
make build
```

To easily run it after building, you can use the following command:
```bash
make run
```
