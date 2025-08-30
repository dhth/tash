<p align="center">
  <h1 align="center">tash</h1>
  <p align="center">
    <a href="https://github.com/dhth/tash/actions/workflows/main.yml"><img alt="GitHub release" src="https://img.shields.io/github/actions/workflow/status/dhth/tash/main.yml?style=flat-square"></a>
    <a href="https://crates.io/crates/tash"><img alt="GitHub release" src="https://img.shields.io/crates/v/tash?style=flat-square"></a>
    <a href="https://github.com/dhth/tash/releases/latest"><img alt="Latest release" src="https://img.shields.io/github/release/dhth/tash.svg?style=flat-square"></a>
    <a href="https://github.com/dhth/tash/releases"><img alt="Commits since latest release" src="https://img.shields.io/github/commits-since/dhth/tash/latest?style=flat-square"></a>
  </p>
</p>

`s[tash]` content that you can access later.

🤔 Motivation
---

Every now and then, I find myself accessing some piece of string content (a
`curl` request, a shell command, or literally anything else) several times over
a period of time. While modern clipboard managers help in recalling previously
copied data, they require some searching. I needed a command line tool that
would make saving and querying string content quick and easy. So, I wrote
`tash`.

💾 Installation
---

**homebrew**:

```sh
brew install dhth/tap/tash
```

**cargo**:

```sh
cargo install tash
```

Or get the binaries directly from a Github [release][1]. Read more about
verifying the authenticity of released artifacts
[here](#-verifying-release-artifacts).

⚡️ Usage
---

### Help

```text
Usage: tash <COMMAND>

Commands:
  delete  Delete one or more content items
  empty   Empty entire stash
  ls      List stashed content keys
  get     Get content from stash
  push    Stash content
  help    Print this message or the help of the given subcommand(s)
```

### Basic Usage

```bash
# push content to tash from stdin
echo -n "some content" | tash push key

cat << EOF | tash push key
Multi line
content goes
here.
EOF

# push content to tash from a file
tash push key -f path/to/file.txt

# push content from a flag
tash push key -d "content goes here"

# push content while preventing overwrites
tash push key -d "content goes here" -p

# push content to tash from system clipboard
tash push key -c

# get content from tash
tash get key

# get content from tash and copy to system clipboard
tash get key -c

# get content from tash and only copy to system clipboard
tash get key -c --no-output

# get content from tash and remove it from its store
tash get key --pop

# list content saved to tash
tash ls

# delete content items
tash delete key1 key2 key3

# empty tash's store
tash empty
```

### Fetch content using fzf

The process of fetching content can be made easier by making use of a fuzzy
finder like [fzf](https://github.com/junegunn/fzf).

```bash
#!/usr/bin/env bash

selected_key=$(
    tash ls |
        fzf \
            --reverse \
            --preview 'tash get {}' \
            --preview-window=right:70% \
            --preview-border=vertical \
            --border=none
)

if [ -z "$selected_key" ]; then
    exit 0
fi

tash get "${selected_key}" -nc
```

### Delete multiple entries using fzf

Same for deletion for content.

```bash
#!/usr/bin/env bash

selected_keys=$(
    tash ls |
        fzf \
            --multi \
            --reverse \
            --preview 'tash get {}' \
            --preview-window=right:70% \
            --preview-border=vertical \
            --border=none |
        xargs
)

if [ -z "$selected_keys" ]; then
    exit 0
fi

tash delete ${selected_keys}
```

🔐 Verifying release artifacts
---

In case you get the `tash` binary directly from a [release][1], you may want
to verify its authenticity. Checksums are applied to all released artifacts, and
the resulting checksum file is attested using [Github Attestations][2].

Steps to verify (replace `A.B.C` in the commands below with the version you
want):

1. Download the sha256 checksum file for your platform from the release:

   ```shell
   curl -sSLO https://github.com/dhth/tash/releases/download/vA.B.C/tash-x86_64-unknown-linux-gnu.tar.xz.sha256
   ```

2. Verify the integrity of the checksum file using [gh][3].

   ```shell
   gh attestation verify tash-x86_64-unknown-linux-gnu.tar.xz.sha256 --repo dhth/tash
   ```

3. Download the compressed archive you want, and validate its checksum:

   ```shell
   curl -sSLO https://github.com/dhth/tash/releases/download/vA.B.C/tash-x86_64-unknown-linux-gnu.tar.xz
   sha256sum --ignore-missing -c tash-x86_64-unknown-linux-gnu.tar.xz.sha256
   ```

3. If checksum validation goes through, uncompress the archive:

   ```shell
   tar -xzf tash-x86_64-unknown-linux-gnu.tar.xz
   cd tash-x86_64-unknown-linux-gnu
   ./tash -h
   # profit!
   ```

[1]: https://github.com/dhth/tash/releases
[2]: https://github.blog/news-insights/product-news/introducing-artifact-attestations-now-in-public-beta/
[3]: https://github.com/cli/cli
