# Tafkars

Tafkars stands for "The API formerly known as...", is written in Rust and is pronounced like "tough cars".

- [`tafkars`](tafkars) contains API bindings for that one site that people used to love.
- [`tafkars-lemmy`](tafkars-lemmy) is an API proxy that allows apps to talk to [Lemmy](https://github.com/LemmyNet/lemmy) through a familiar API from a kinder time.

## Help wanted

There is still lots of work to be done, especially on [`tafkars-lemmy`](tafkars-lemmy). Pull requests welcome!

## Quickstart Guide

This Quickstart guide assumes you are using [this fork of libreddit](https://github.com/derivator/libreddit) and [lemmy.world](lemmy.world) to quickly get up and running with tafkars.

- Run tafkars:
    - You can substitute lemmy.ml with any lemmy instance of your choice
```
git clone https://github.com/derivator/tafkars
cd tafkars
cargo run lemmy.world
```
- Run libreddit app against `127.0.0.1:8000` (default location of tafkars API):
    - You can substitute this with any app that allows for a custom API URL to be passed in
```
git clone https://github.com/derivator/libreddit
cd libreddit
cargo run
```
- Visit webclient for libreddit: [http://localhost:8080/](http://localhost:8080/)
    - You can substitute this with whatever app you are using.