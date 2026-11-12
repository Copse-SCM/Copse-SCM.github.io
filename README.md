# copse-site

The website for [Copse](https://github.com/Copse-SCM/copse), written in Rust:
[Leptos](https://leptos.dev) rendered in the browser (CSR), compiled to WebAssembly and
built with [Trunk](https://trunkrs.dev).

## Run it

You need Rust with the `wasm32-unknown-unknown` target (`rust-toolchain.toml` asks for it)
and Trunk (`brew install trunk`).

```sh
trunk serve             # http://127.0.0.1:8080, rebuilds on save
trunk build --release   # the finished site, in dist/
```

## Layout

- `src/app.rs`: the router. Pages live at `/`, `/install`, `/guide`, `/releases` and
  `/security`; the old `*.html` addresses redirect to them.
- `src/pages/`: one component per page. Their content was carried over from the HTML site
  unchanged.
- `src/components/`: the header and footer, and `CopyBlock`, a code block with a Copy
  button (it leaves trailing `# comments` out of what it copies).
- `style/`: the stylesheet as SCSS partials, one per part of the site, pulled together by
  `main.scss`. Trunk compiles it.
- `public/images/`: the logo and favicon, copied into the build as they are.

## Publishing

`.github/workflows/pages.yml` builds the site with Trunk and deploys it to GitHub Pages
on each push to `main`. It also saves `index.html` as `guide.html`, `install.html`,
`releases.html` and `security.html`, which Pages serves for `/guide` and the rest with
a proper 200 status, and as `404.html` for any other address. The repository's Pages
source must be set to "GitHub Actions" (Settings → Pages) before the first deploy.

Design: a light theme, a Times New Roman–style serif, and the logo's greens
(#0C5C27, #3DBA6E). The commit graph appears only in its one illustrated section.
