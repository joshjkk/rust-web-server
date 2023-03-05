# rust web server

A web server written in Rust.

![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/joshjkk/rust-web-server?include_prereleases)
![GitHub](https://img.shields.io/github/license/joshjkk/rust-web-server)

## Installation and usage 

Build and run using Cargo, you can optionally use the ```--release``` flag for a significantly smaller executable size and better program speed.

## Rendering HTML

- HTML files are placed in the ```templates``` directory 

- Stylesheets are placed in the ```static``` directory

- JavaScript files are placed in the ```scripts``` directory

**Currently, only ```index.html``` and ```404.html``` are rendered**, but routing for all HTML files will be added soon.

## License

rust web server is licensed under an Apache 2.0 license.
