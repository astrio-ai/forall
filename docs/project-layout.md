# Project Layout

`forall init` creates a `.forall/` directory at the root of your project. Its
presence marks the project root, and it holds everything Forall needs to track
specs and verification for that repository.

```text
.forall/
├── markers.toml           # marks the project root
├── workflow/
│   └── config.yaml        # workflow schema and project rules
└── verify/
    └── mapping.yaml       # requirement → code mapping
```

## Files

### `markers.toml`

Tells the Forall agent that this directory is a project root.

```toml
project_root_markers = [".forall/verify/mapping.yaml"]
```

### `workflow/config.yaml`

Selects the workflow schema and carries project-level context and rules.

```yaml
schema: forall
context: |
  This project uses the Forall workflow. Write specs before code.
  Run `forall check` before archive.
rules:
  verification:
    - Run `forall check --change <name>` and fix all CRITICAL issues before archive
```

### `verify/mapping.yaml`

Maps requirements to the code that satisfies them. It starts empty and grows as
you add verified requirements.

```yaml
version: 1
requirements: []
```

## Generated during work

Once you start a change, Forall adds working directories under `.forall/`:

```text
.forall/
└── workflow/
    ├── changes/<name>/    # an in-progress change (proposal, specs, tasks, …)
    └── archive/           # completed changes
```

## Should I commit `.forall/`?

Yes. `.forall/` is shareable project configuration — commit it so your whole
team works against the same specs and rules. Machine-local state lives in your
home directory (`~/.forall/`), not in the project, and should not be committed.
