# Workflow

Forall encourages a spec-driven change workflow: describe what you want, write
it down, implement it, and verify before you finish.

```text
propose → (specs / design) → apply → verify → archive
```

## 1. Propose

Start a named change. This creates a working directory under
`.forall/workflow/changes/<name>/` with a proposal and supporting artifacts.

```bash
forall propose <name>
```

## 2. Write specs

Capture requirements before writing code. Specs live inside the change
directory and describe the behavior you expect.

## 3. Apply

Implement the change. Forall works alongside your edits and keeps the
requirement mapping in sync.

## 4. Verify

Run the check to confirm the change meets its requirements:

```bash
forall check --change <name>
```

Fix any CRITICAL issues before moving on.

## 5. Archive

When the change is complete and checks pass, archive it. This records the
result and moves the change into `.forall/workflow/archive/`.

```bash
forall archive <name>
```

---

> The exact set of artifacts and checks depends on your project configuration in
> `.forall/workflow/config.yaml`.
