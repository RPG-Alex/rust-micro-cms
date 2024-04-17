<a name="readme-top"></a>
# Rust Micro CMS Frontend
<div align="center">
<img src="../images/logo.png" alt="Logo" width="80" height="80">
</div>
<div align="center">
<img src="../images/Rust-Micro-CMS-Structure.png" alt="Project Structure">
</div>
<details>
  <summary>Contents</summary>
  <ol>
    <li>
      <a href="#about-this-frontend">About this Frontend</a>
    </li>
    <li>
      <a href="#installation">Installation</a>
    </li>
    <li>
      <a href="#running">Running</a>
    </li>
    <li>
      <a href="#contributing">Contributing</a>
    </li>
  </ol>
</details>

## About this Frontend

Rust Micro CMS Frontend is a streamlined interface designed for the Rust Micro CMS, focusing on ease of use and customization. It leverages the power of Yew, a modern Rust framework, to deliver a responsive and efficient user experience. This frontend is connected to a RESTful API provided by the Rust Micro CMS backend, allowing dynamic content management and real-time interactions.

### Installation

You need Rust, Cargo, and the `wasm32-unknown-unknown` target installed for Yew development. If not already installed, follow these steps:

1. **Install Rust and Cargo:**
   Install Rust and Cargo using rustup, Rust's official installer. If you haven't installed Rust yet, you can do so by visiting:
   ```plaintext
   https://www.rust-lang.org/tools/install
   ```

2. **Add the WASM Target:**
   Yew compiles to WebAssembly. Add the necessary target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Install Trunk:**
   Trunk simplifies Yew application builds and asset management:
   ```bash
   cargo install trunk wasm-bindgen-cli
   ```

### Running

To run the frontend locally:

1. **Start the development server:**
   ```bash
   trunk serve
   ```
   This command will build the app, watch for changes, and serve it locally.

2. **Alternative - Build only:**
   For a production build:
   ```bash
   trunk build --release
   ```
   Output will be in the `dist` directory.

## Contributing

Contributions to Rust Micro CMS Frontend are welcome! Whether it's feature improvements, bug fixes, or documentation, your input is valuable. Here's how you can contribute:

1. **Fork the repository:** Start by forking the Rust Micro CMS Frontend repository on GitHub.

2. **Clone your fork:** Clone your forked repository to your local machine to start making changes.
   ```bash
   git clone https://github.com/your-username/rust-micro-cms-frontend.git
   cd rust-micro-cms-frontend
   ```

3. **Create a new branch:** It's best to create a branch for each set of changes you make.
   ```bash
   git checkout -b your-new-feature
   ```

4. **Make changes:** Update or add new features to the frontend. Be sure to test your changes thoroughly.

5. **Commit your changes:** Once you're happy with the changes, commit them.
   ```bash
   git commit -am 'Add some feature'
   ```

6. **Push to your fork:** Push your changes to your fork on GitHub.
   ```bash
   git push origin your-new-feature
   ```

7. **Submit a pull request:** Go to the original Rust Micro CMS Frontend repository and submit a pull request from your feature branch to the main branch.

Before contributing, please check the project's issues and pull requests to see if someone else has already suggested a similar change.

<p align="center">[<a href="#readme-top">RETURN TO TOP</a>]</p>
