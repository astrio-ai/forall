# Hosted Forall MCP

## Purpose

The hosted Forall MCP server runs Forall verification remotely in isolated
workers. It serves two product paths:

- standalone Forall exposes native verification tools backed by this service;
- external coding agents connect to the service through its public MCP tools.

The hosted service never writes a user's local workspace. Local authoring is
handled in-process by standalone Forall or through the separate
[local authoring MCP](local-authoring-mcp.md) for external clients.

## Scope

The first release:

- uses MCP Streamable HTTP at `https://mcp.forall.astrio.app/mcp`;
- authenticates clients with Forall API keys;
- accepts selected files supplied by an MCP client;
- accepts public GitHub repositories and resolves refs to immutable commits;
- runs full verification asynchronously in an isolated worker;
- returns sanitized structured reports;
- supports job status, cancellation, and result explanation.

Private GitHub repositories, OAuth, and spec generation are deferred.

## Architecture

```text
Standalone native verify tools or external MCP client
    │  Streamable HTTP + API key
    ▼
Hosted Forall MCP service
    │
    └──────────► isolated verification worker
                      │
                      ▼
              Forall verification pipeline
```

Verification is asynchronous because proofs and property tests can take longer
than MCP and proxy request timeouts. Clients submit a job and poll for its
status and result.

Standalone Forall presents this as native `verify.*` tools and manages the
connection, source packaging, and authentication internally. Users of the
standalone agent should not manually configure this MCP endpoint.

The open-source client library in `crates/forall-hosted-verify` implements
snapshot packaging, authenticated MCP calls, and response parsing. It does not
contain the hosted service itself.

The standalone credential flow reads the key from standard input and stores it
through Forall's local secret store:

```bash
printenv FORALL_API_KEY | forall verification login
```

## Client and worker dependencies

The hosted MCP is a service URL, not software installed on the user's machine.
Clients need only:

- standalone Forall, or an MCP-capable external coding agent;
- a Forall API key;
- the source files or public GitHub reference to verify.

Clients do not install language proof engines for hosted jobs. The isolated
worker image owns the complete verification runtime, including the language
toolchains used by the workflow. Production images must provision and pin these
dependencies during image construction; jobs must not depend on modifying the
user's machine or discovering tools from it.

This boundary applies to both product paths:

```text
Standalone Forall
    native verify.submit → hosted worker toolchains

Cursor / Claude Code / Codex
    hosted MCP forall_verify → hosted worker toolchains
```

Local toolchains are required only for the separate local `forall check`
pipeline. Installing the Forall executable does not eagerly install every
proof engine; see [Architecture](architecture.md#distribution-and-proof-toolchains).

## Source inputs

`forall_verify` accepts one of two source variants.

### Inline files

```json
{
  "type": "inline",
  "files": [
    {
      "path": "src/lib.rs",
      "content": "..."
    }
  ]
}
```

The service validates file count, file and total size, encoding, path traversal,
symlinks, and archive expansion before dispatch.

The `SnapshotPacker` in `crates/forall-hosted-verify` builds inline snapshots
from a local workspace for clients that submit from disk.

### GitHub repository

```json
{
  "type": "github",
  "repository": "owner/repository",
  "ref": "main",
  "subdirectory": "optional/path"
}
```

The service resolves the ref to an immutable commit and records that revision
with the job. The MVP accepts public repositories only and does not run Git
hooks or fetch submodules or Git LFS objects.

## MCP tools

### `forall_verify`

Submits whole-project or change-scoped verification.

Input:

```json
{
  "source": {
    "type": "inline",
    "files": [
      {
        "path": "src/lib.rs",
        "content": "..."
      }
    ]
  },
  "scope": {
    "type": "change",
    "name": "add-rate-limit"
  },
  "strict": true,
  "phases": [
    "structure",
    "mapping",
    "proofs",
    "intent",
    "scenarios",
    "property-tests",
    "scenario-tests"
  ],
  "pbt_seed": 42,
  "pbt_examples": 100
}
```

`scope.type` is either `project` or `change`. A change scope requires `name`.
The server applies its own maximum property-test count and job duration.

Response:

```json
{
  "job_id": "vrf_...",
  "status": "queued",
  "submitted_at": "2026-07-12T00:00:00Z",
  "source_revision": "optional resolved commit SHA",
  "poll_after_ms": 2000
}
```

### `forall_verification_status`

Returns progress and, when complete, the verification result.

Input:

```json
{
  "job_id": "vrf_..."
}
```

Possible states are `queued`, `preparing`, `running`, `succeeded`, `failed`,
`cancelled`, and `expired`.

Completed response:

```json
{
  "job_id": "vrf_...",
  "status": "succeeded",
  "progress": {
    "phase": "proofs",
    "completed": 3,
    "total": 7
  },
  "result": {
    "ok": false,
    "strict": true,
    "phases": {
      "mapping": "PASS",
      "proofs": "FAIL"
    },
    "issues": [
      {
        "severity": "CRITICAL",
        "phase": "proofs",
        "requirement_id": "REQ-1",
        "message": "...",
        "file": "src/lib.rs",
        "counterexample": null,
        "proof_detail": "sanitized output"
      }
    ],
    "verified_files": [
      "src/lib.rs"
    ],
    "verification_summary": {
      "total_requirements": 1,
      "proved_requirements": 0,
      "property_tested_requirements": 0,
      "spec_tracked_requirements": 1
    }
  },
  "error": null
}
```

The public report excludes container paths, environment values, credentials,
and unrestricted command lines.

### `forall_cancel_verification`

Cancels a queued or running job. Cancellation is idempotent.

Input:

```json
{
  "job_id": "vrf_..."
}
```

### `forall_explain_verification`

Explains selected issues from a completed report and suggests concrete
remediation without rerunning project code.

Input:

```json
{
  "job_id": "vrf_...",
  "issue_indexes": [
    0
  ],
  "audience": "concise"
}
```

`audience` is either `concise` or `detailed`.

## Sandbox requirements

Project code and verification inputs are untrusted. Every job runs in a fresh
isolated worker with:

- a non-root verification process;
- bounded scratch storage and fixed CPU, memory, process, and wall-clock limits;
- no general outbound network access;
- preinstalled, pinned proof toolchains;
- single-job, short-lived input and output access;
- separate protected report output owned by the supervisor.

Inputs and results are encrypted at rest and removed after a short retention
period. Logs and reports redact credentials, absolute container paths, and
environment values.

## Client library

The `forall-hosted-verify` crate exposes:

- `HostedVerificationClient` for MCP initialize, submit, status, cancel, and explain;
- `SnapshotPacker` for safe workspace snapshots;
- wire types in `dto` for requests and responses.

Run tests from the repository root:

```bash
cargo test -p forall-hosted-verify
```

The integration test in `tests/standalone_flow.rs` exercises authoring,
validation, snapshot packaging, and hosted submit end-to-end against a mock MCP
server.
