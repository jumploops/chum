const fs = require("node:fs");
const path = require("node:path");
const { targetTriple } = require("./resolve-binary");

const triple = targetTriple();
if (!triple) {
  console.error(`@magicloops/chum does not yet ship a binary for ${process.platform}/${process.arch}.`);
  console.error("Supported platforms are macOS and Linux on arm64/x64. You can also try `cargo install chum`.");
  process.exit(0);
}

const vendorDir = path.join(__dirname, "..", "vendor", triple);
const binary = path.join(vendorDir, process.platform === "win32" ? "chum.exe" : "chum");

if (!fs.existsSync(binary)) {
  fs.mkdirSync(vendorDir, { recursive: true });
  console.error(`@magicloops/chum installed without bundled binary ${triple}.`);
  console.error("Release packaging should place the native chum binary here.");
  console.error("Until then, use `cargo install chum` or a local build.");
}
