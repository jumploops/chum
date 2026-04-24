const fs = require("node:fs");
const path = require("node:path");

function targetTriple(platform = process.platform, arch = process.arch) {
  if (platform === "darwin" && arch === "arm64") return "aarch64-apple-darwin";
  if (platform === "darwin" && arch === "x64") return "x86_64-apple-darwin";
  if (platform === "linux" && arch === "arm64") return "aarch64-unknown-linux-gnu";
  if (platform === "linux" && arch === "x64") return "x86_64-unknown-linux-gnu";
  return null;
}

function resolveBinary() {
  const triple = targetTriple();
  if (!triple) {
    throw new Error(
      `unsupported platform ${process.platform}/${process.arch}; supported platforms are macOS and Linux on arm64/x64`
    );
  }
  const binary = path.join(__dirname, "..", "vendor", triple, process.platform === "win32" ? "chum.exe" : "chum");
  if (!fs.existsSync(binary)) {
    throw new Error(`missing chum binary for ${triple}; reinstall @magicloops/chum or run \`cargo install chum\``);
  }
  return binary;
}

module.exports = { resolveBinary, targetTriple };
