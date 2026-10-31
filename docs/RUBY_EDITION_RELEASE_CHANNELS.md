# Ruby Edition download channels for Ruby Launcher

This repository publishes **compiled game binaries**, not Mojang's game assets. The Windows and Linux builds run on GitHub Actions using [Ruby Edition game packages](../.github/workflows/ruby-edition-packages.yml).

## Branch responsibility

- `main` — **Horizon integration**. Merge upstream Horizon's `main` here, preserving its commit ancestry and retaining the Ruby Edition packaging and integration documentation. This branch is **not** a public Ruby game release channel.
- `nightly` — **Ruby game development**. Keeps the game branding, bug fixes, and experimental changes; selectively picks reviewed upstream changes from `main`.
- `stable` — **Ruby Stable**. Receives only approved, tested game changes, separate from integration and ongoing development.

Horizon includes its own launcher source, but the Ruby Edition packaging workflow builds only the **game** and resource-preparation tools. **Ruby Launcher** is developed and distributed from its own `SirSkirt/Ruby-Launcher` repository; do not publish Horizon's launcher as Ruby Launcher.

## Channel contract

| Launcher selection | Source branch | Release tag prefix | GitHub release status |
| --- | --- | --- | --- |
| Stable | `stable` | `ruby-edition-stable-` | Normal release (marked Latest) |
| Nightly | `nightly` | `ruby-edition-nightly-` | Prerelease |

Tags are **unique per build** and must not be force-moved. Examples: `ruby-edition-stable-2-12345678` and `ruby-edition-nightly-3-87654321`.

Use the GitHub Releases API rather than guessing a download URL:

- Stable: `GET https://api.github.com/repos/SirSkirt/ruby-edtion/releases/latest`; verify tag starts with `ruby-edition-stable-`.
- Nightly: `GET https://api.github.com/repos/SirSkirt/ruby-edtion/releases?per_page=100`; select the newest published release whose tag starts with `ruby-edition-nightly-`, `prerelease` is true, and `draft` is false.
- If there is no matching release yet, show **Not available** rather than silently substituting the other channel.

## Assets and layout

Each published release has these assets:

- `ruby-edition-linux-x86_64.zip`
- `ruby-edition-windows-x64.zip`
- `SHA256SUMS`

Each ZIP contains a `ruby-edition/` folder:

- `horizon-client` (Windows: `horizon-client.exe`) — game executable
- `horizon-prepare` (Windows: `horizon-prepare.exe`) — resource preparation helper
- `horizon-ctl` (Windows: `horizon-ctl.exe`) — optional command helper
- `ruby-edition-build.json` — `format_version`, `channel` (stable/nightly), `commit`, `platform`, `minecraft_version`
- `research-lock.json` and required resource-preparation sources

The ZIP names are intentionally the **same in each channel**; the release tag identifies the channel/build. Validate `SHA256SUMS` before extraction, and verify `ruby-edition-build.json` matches the requested channel and platform. Reject unsafe archive paths. Keep Stable and Nightly installs, settings, and worlds separate. Install to a temporary directory, then replace the selected channel atomically.

## Resource preparation

**Minecraft 26.3 resources are NOT packaged.** The game requires a legitimately owned Minecraft Java Edition installation, Mojang's official resources, and Horizon's local preparation process. The current upstream preparation flow may require additional prerequisites such as JDK 25 and the game/server files; launcher integration must handle or explain these prerequisites. Do not publish Minecraft assets or prepared `.local/` data.

A release does not mean the game is playable out of the box before local resource preparation.

## Publishing

- Push to `nightly` -> automatically builds and publishes a prerelease after compilation and the save-persistence checks pass.
- Push to `stable` -> automatically builds and publishes a Stable release after checks pass.
- The workflow can also be run manually with **Actions**. A manual workflow must be started on the desired source branch.
- Release artifacts are public; no GitHub token should be embedded in Ruby Launcher for reading public releases.
- `main` integrates upstream Horizon changes and runs verification, but never publishes Ruby Edition game releases. Reviewed changes can be ported to `nightly`, then promoted to `stable` after testing. Keep Ruby branding and distribution rules intact.

## Testing Save and Quit

The Nightly source contains a fix choosing player persistence from the game execution role rather than `options.dedicated`. The integrated server uses the latter flag for internal reasons. Regression tests cover the role rule and player/world data roundtrip. A proper end-to-end manual test should:

1. Back up or create a disposable world.
2. Change blocks, collect items, move away from spawn, and gain XP.
3. Save and Quit to Title, reopen the same world.
4. Confirm world blocks, position, inventory, XP, and game time persist.
5. Repeat after a full process exit.

The draft Save and Quit PR remains separate from `main` pending runtime testing.
