# tash

[![Build Workflow Status](https://img.shields.io/github/actions/workflow/status/dhth/tash/build.yml?style=flat-square)](https://github.com/dhth/tash/actions/workflows/build.yml)
[![Tests Workflow Status](https://img.shields.io/github/actions/workflow/status/dhth/tash/test.yml?style=flat-square&label=tests)](https://github.com/dhth/tash/actions/workflows/test.yml)

`tash` "stashes" content that you can access later.

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

**cargo**:

```sh
cargo install --git https://github.com/dhth/tash.git
```

⚡️ Usage
---

### Help

```text
Usage: tash <COMMAND>

Commands:
  e     Empty entire stash
  ls    List stashed content keys
  g     Get content from stash
  p     Push content to stash
  help  Print this message or the help of the given subcommand(s)
```

### Basic Usage

```bash
# push content to tash from stdin
echo -n "some content" | tash p key

cat << EOF | tash p key
Multi line
content goes
here.
EOF

# push content to tash from a file
tash p key -f path/to/file.txt

# push content from a flag
tash p key -d "content goes here"

# push content while preventing overwrites
tash p key -d "content goes here" -p

# push content to tash from system clipboard
tash p key -c

# get content from tash
tash g key

# get content from tash and copy to system clipboard
tash g key -c

# get content from tash and only copy to system clipboard
tash g key -c --no-output

# get content from tash and remove it from its store
tash g key --pop

# list content saved to tash
tash ls

# empty tash's store
tash e
```
