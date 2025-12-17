# Seems Good Webpage Repository
## Local Setup Prereqs/Dependencies:
- [install rust](https://rustup.rs/)
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

### List of Resources:
- [Render HTML file with Axum](https://github.com/programatik29/axum-tutorial)
- [Cloudflare Worker Docs](https://developers.cloudflare.com/workers/)
- [Askama Templating Docs](https://djc.github.io/askama/)

## Libs/Software Used:
 - Bulma (css)
 - Axum (routing, handling http)
 - Cloudflare workers


Made with  in 
