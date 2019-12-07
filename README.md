
# Svelte-on-Rust

Starter template for [Svelte](https://svelte.dev) apps with Rust [Rocket](https://rocket.rs)) Server . 

To create a new project based on this template using [degit](https://github.com/Rich-Harris/degit):

*Requirements*
    NodeJs - [Install](https://nodejs.org/en/download/)
    Rust  - [Install](https://www.rust-lang.org/tools/install) 
    Rust Nightly for the project folder


## Get started

Install the dependencies...

```bash
npx degit sachinbhutani/svelte-on-rust svelte-rocket
cd svelte-rocket
rustup override set nightly
npm install
```

...then start [Rollup](https://rollupjs.org):

```bash
npm run dev
```

Navigate to [localhost:8000](http://localhost:8000). You should see your app running. 
All svelte component live in `client` directory. Save any changes live-reloading.
All Rocket code lives in `src` directory. To rebuild Rust code use cargo run after saving your changes. 
All static files are served from `public` direcotry. Including the JS code compiled by Svelte Compiler.


## Building and running in production mode

To create an optimised version of the app:

```bash
npm run build
cargo build
```