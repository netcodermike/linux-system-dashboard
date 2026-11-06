# Validation record

Authoring host: Windows, November 6, 2026. Requirements: Rust stable, edition 2021; Linux only.

The README verification command is `cargo test`. The companion GitHub Actions workflow describes reproducible build/test checks. Merely including that workflow does not mean it ran successfully. Host-specific verification results are recorded before publication.

Manual acceptance steps are in README.md. GUI behavior, hardware sensors, permissions, audio output, and OS isolation need verification on their target platform; compilation alone cannot establish these properties.

## Authoring result

Target toolchain/platform unavailable on the Windows authoring host. Source and build configuration were reviewed, and a target-platform GitHub Actions build/check workflow is provided. No successful native build or runtime result is claimed yet.

## Observed GitHub Actions result

Passed on November 6, 2026: [build/check run](https://github.com/netcodermike/linux-system-dashboard/actions/runs/42196076730).

Verified source commit: `3354fa6284ba134a9c5f9cddde85f749a1d06c2c`.

Rust parser tests plus a live Linux one-sample dashboard run.

This record may be committed after the linked source-validation run; the linked SHA identifies exactly what was checked. Native GUI/hardware acceptance remains manual as described in README.md.
