# tafkars-lemmy

Tafkars stands for "The API formerly known as...", is written in Rust and is pronounced like "tough cars".  
`tafkars-lemmy` is an API proxy that allows apps to talk to [Lemmy](https://github.com/LemmyNet/lemmy) through a familiar API from a kinder time.  
The hope is that this will make it easy for app developers to support Lemmy with only minimal code changes.  

## API implementation status

- [ ] viewing:
    - [ ] pagination
    - [ ] sorting
    - [x] community post list
    - [ ] community info (sidebar, mods, etc.)
    - [x] posts
    - [x] comments
    - [x] comment threading
    - [ ] user profiles
    - [ ] user post/comment list
    - [ ] inbox
    - [ ] moderation queue
- [ ] posting/interacting:
    - [ ] login
    - [ ] voting
    - [ ] posting
    - [ ] commenting
    - [ ] direct messages
    - [ ] moderation

## Help wanted

There is still lots of work to be done. Pull requests welcome!  
If you can't code there's still lots of ways you can help:
- check if your favourite open source 3rd party app can be modified to send API requests to a different URL
- test if all of the [implemented features](#api-implementation-status) are working, [report](https://github.com/derivator/tafkars/issues) any that aren't
- write documentation

Please contact me here or [on Lemmy](https://feddit.de/u/derivator) if you are:
- a Lemmy instance operator that needs help running this on your instance
- an app developer that wants to support this

Please **do not**:
- spam app developers demanding support for this
- spam Lemmy instance operators to run this
- point a public instance of this to a Lemmy instance without the operator's consent

## Testing
Clone the repository, execute
```
cargo run
```
and follow the instructions.
You need an app that allows configuration of the API url. For now, there's a [fork of libreddit that connects to `localhost`](https://github.com/derivator/libreddit) that you can use for testing.