#!/usr/bin/env node

const { spawnSync } = require("node:child_process");
const { resolveBinary } = require("../scripts/resolve-binary");

const binary = resolveBinary();
const result = spawnSync(binary, process.argv.slice(2), { stdio: "inherit" });

if (result.error) {
  console.error(`failed to execute chum binary: ${result.error.message}`);
  console.error("Try reinstalling @magicloops/chum or run `cargo install chum`.");
  process.exit(1);
}

process.exit(result.status ?? 1);
