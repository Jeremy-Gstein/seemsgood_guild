<<<<<<< HEAD
# Seems Good Webpage Repository

[![Rust CI](https://github.com/Jeremy-Gstein/seemsgood_guild/actions/workflows/ci.yml/badge.svg)](https://github.com/Jeremy-Gstein/seemsgood_guild/actions/workflows/ci.yml)
=======
# Development Branch
- switching to Cloudflare as serverless host provider.
- did not support actix_web - changing to Axum

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

## Templating:
- Using askama_axum (askama) as templating engine
- Todo: { Add example docs }
>>>>>>> origin/devel
