/*
Copyright 2024-2025 The Spice.ai OSS Authors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

     https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::process::Command;

fn main() {
    let git_hash: String = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map_or_else(
            |_| "unknown".to_string(),
            |output| String::from_utf8_lossy(&output.stdout).trim().to_string(),
        );

    println!("cargo:rustc-env=GIT_COMMIT_HASH={git_hash}");

    // Force this build script to re-run on every build, even when neither the
    // sources nor the current commit changed since the last build. Cargo only
    // re-runs build scripts when a package file changes by default, which can
    // leave GIT_COMMIT_HASH stale (e.g. after `git commit --amend`, a checkout,
    // or a rebase that doesn't touch this crate's files).
    //
    // Pointing `rerun-if-changed` at a path that never exists makes Cargo treat
    // the build-script fingerprint as perpetually dirty, so the script runs on
    // every build. (A stamp file in OUT_DIR does NOT work: once written, its
    // mtime stops changing, so Cargo would consider it up to date.)
    println!("cargo:rerun-if-changed=.cargo-always-rebuild");
}
