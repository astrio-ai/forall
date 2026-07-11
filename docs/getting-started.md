# Getting Started

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/astrio-ai/forall/main/install.sh | bash
```

The installer downloads the prebuilt `forall` binary for your platform into
`~/.local/bin`. If that directory is not on your `PATH`, add it:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Verify the install:

```bash
forall --version
```

### Supported platforms

| OS | Architectures |
|----|---------------|
| macOS | Apple Silicon (`aarch64`), Intel (`x86_64`) |
| Linux | `x86_64`, `aarch64` |
| Windows | `x86_64` |

## Initialize a project

From the root of a git repository:

```bash
forall init
```

This scaffolds a `.forall/` directory that marks the project root and holds the
workflow and verification config. `forall init` is language-neutral — it only
creates the workflow files.

See [Project Layout](project-layout.md) for what gets created.

## Start working

```bash
forall
```

From here, use the workflow to make changes with specs first. See
[Workflow](workflow.md).
