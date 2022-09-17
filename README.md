# rust-api-workshop
Write a production-ready Rust HTTP API

Some pieces of code, names or conventions are inspired by [zero-to-production](https://github.com/LukeMathWalker/zero-to-production).

## What will you build?

A simple HTTP API that checks if Yoda is taller than a given Star Wars character.

For example, if we want to check if Yoda is taller than Luke Skywalker,
we can do this GET request with `curl`:

```sh
$ curl 127.0.0.1:3000/taller/luke
{
  "query": "luke",
  "person": "Luke Skywalker",
  "taller": false
}
```
