# Seems Good Webpage Repository
[![rust-clippy analyze](https://github.com/Jeremy-Gstein/seemsgood_guild/actions/workflows/rust-clippy.yml/badge.svg?branch=main)](https://github.com/Jeremy-Gstein/seemsgood_guild/actions/workflows/rust-clippy.yml)
## Prereqs/Dependencies:
- install wasm target system
  `rustup install wasm32-unknown-unknown`
- [install npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- install npx package:
  `npm install -g npx`
- clone the repo:
  `git clone https://github.com/Jeremy-Gstein/seemsgood_guild.git`
- setup local server:
    `npx wrangler dev`
- deploy local to cloudflare cdn:
  `npx wrangler deploy`

### TODO:
- ~~add templating to avoid repetitive html~~
- Start styling the website and adding relevent information.

### List of Resources:
- [Render HTML file with Axum](https://github.com/programatik29/axum-tutorial)
- [Cloudflare Worker Docs](https://developers.cloudflare.com/workers/)
- [Askama Templating Docs](https://djc.github.io/askama/)

## Templating:
- Using askama_axum (askama) as templating engine
- Todo: { Add example docs }
