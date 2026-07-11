# hello-forall

Minimal example of a Forall project layout after `forall init`.

The `.forall/` directory is the project root marker and holds workflow + verification config. Copy it into a new repo, or run:

```bash
forall init
```

## Layout

```text
.forall/
├── markers.toml           # tells Forall this is a project root
├── workflow/
│   └── config.yaml        # workflow schema and project rules
└── verify/
    └── mapping.yaml       # requirement → code mapping (empty to start)
```

Active changes live under `.forall/workflow/changes/` once you run `forall propose`.
