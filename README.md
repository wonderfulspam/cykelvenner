# Cykelvenner

Where the software delivery life cycle meets the life of cycling.

## Development

In separate terminals:

```bash
cd frontend && npm install && npm run dev
```

```bash
cargo run
```

Now navigate to <http://localhost:3000>.

### Requirements

* `npm`
* `rust`

### Using `just`

A [`justfile`](./justfile) is provided. Install
[Just](https://github.com/casey/just/) and run `just --list` for a list of
available commands.

## Architecture

`Cykelvenner` is split into a distinct frontend and backend. Neither one knows
much about the other; the frontend lives in [`frontend/`](./frontend/) and
simply knows that its generated assets (HTML pages, JavaScript files, CSS) will
be served by _some_ backend, and that the backend will be available at `/api/`.
Similarly, the backend simply serves the files in `frontend/dist/` and is
blissfully unaware of the visual representation of the data it provides at
`/api/`.

This decoupling allows using completely different languages, paradigms and tools
for the job without worrying about one part of the system breaking the other.
Additionally, a statically built frontend allows serving assets from a CDN,
greatly optimizing time to first byte and massively reducing hosting complexity
as the frontend can be hosted from anywhere ([CORS][1] permitting).

### Frontend

The user-facing parts of the application are built in HTML/TypeScript/CSS using
[Svelte](https://svelte.dev/), a frontend framework with a minimal footprint
thanks to its compiler-based architecture. Svelte's compiler is invoked by
[Vite](https://vitejs.dev/), a toolkit for lean frontend development which
also handles the bundling of assets and provides a hot-reloading dev server for
local development.

### Backend

The backend is built with [Axum](https://github.com/tokio-rs/axum/), a web
framework designed for simplicity, modularity and ergonomics.

### Delivery

The application runs as a Heroku app with only a few lines of configuration in
[`.buildpacks`](./.buildpacks) and [`Procfile`](./Procfile).

### Goals

The application is an attempted marriage between the [Twelve-Factor
App](https://12factor.net/) and [Jamstack](https://jamstack.org/). It aims for
simplicity, efficiency and safety by leveraging performant, well-designed
frameworks, libraries and platforms.

## Credits

[Svelte + TS + Vite template][2] by `dsegovia90`.

[1]: https://en.wikipedia.org/wiki/Cross-origin_resource_sharing
[2]: https://github.com/dsegovia90/wasm-vite-svelte-monorepo/tree/master/packages/web

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
