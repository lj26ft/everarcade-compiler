# Local developer journey

Follow the maintained [local developer quickstart](../developer/quickstart.md) for the source-pinned clone, Node 24.15.0/Rust 1.95.0 prerequisites, checksum-verified offline vendor restoration, disposable arena project, package, native local session and replay inspection.

Allow extra time for toolchain installation and the first native compile. The source `new/build/test` steps alone check template metadata and source; only successful packaging and `play-local` establish the bounded local native path. They do not establish production or independent acceptance.

The quickstart also documents a bounded executable helper, retained logs and troubleshooting. The full Contributor Gate remains the commands in [CONTRIBUTING.md](../../CONTRIBUTING.md); the World Artifact Gate remains separate. Existing onboarding scripts write reports under tracked paths, so inspect their scope before running them in an active worker checkout.
