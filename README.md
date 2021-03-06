# fast-rpc

This project aims to implement the Remote Procedure Call (RPC) framework.

# Getting Started

## Prerequisites

This is primarily a Rust project. Therefore, it is important to download rust on your operating device.
- Rust (Linux or macOS)
> $curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
- Additional step for macOS to install a C compiler to include a linker
> xcode-select --install
- Rust (Windows)
> follow instructions on [rust installation website]( https://www.rust-lang.org/tools/install)

- Update Rust
> rustc --version

For further clarification, refer to this Rust [installation guide](https://doc.rust-lang.org/book/ch01-01-installation.html)
## Installation
1. Open a terminal and navigate to the project. Afterwards, run the following command:
> cargo build

2. Once the project has been built, you can safely run the webserver with the following command:
> cargo run

3. Upon successful startup, open a web browser and connect to http://127.0.0.1:7878/.

    The expected outcome is following:

    ![Expected outcome](https://user-images.githubusercontent.com/76085494/159887493-613bc529-b3bb-4cc2-a154-b176f99e24a5.PNG "Expected Outcome")

## Usages
_To be continued_

## Roadmap
- [x] Setup skeletal code for multithreaded processes to accommodate request traffic.
- [ ] Implement graceful shutdown and cleanup from tutorial.
- [ ] Start with hyperium/tonic implementation of gRPC.
- [ ] Create custom API endpoints for web services.
- [ ] Create test cases for API methods.
- [ ] Integrate Swagger for API visual testing.

## Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

Fork the Project
Create your Feature Branch (git checkout -b feature/FeatureName)
Commit your Changes (git commit -m 'Add some FeatureName')
Push to the Branch (git push origin feature/FeatureName)
Open a Pull Request

## License
_To be continued_

## Contact
Joshua - [LinkedIn](https://www.linkedin.com/in/joshuayap98/) [Instagram](https://www.instagram.com/ywjj_/?hl=en)

## Acknowledgments
1. [hyperium/tonic](https://github.com/hyperium/tonic)
2. [Rust Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
