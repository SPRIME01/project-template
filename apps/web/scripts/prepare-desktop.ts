import { createHash } from "node:crypto";
import { chmod, mkdir, readFile, rename, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";

const tag = "v0.15.2";
const release = "https://github.com/vectorize-io/pg0/releases/download/" + tag + "/";
const binaries: Record<string, { name: string; sha256: string }> = {
  "linux-x64": { name: "pg0-linux-x86_64-gnu", sha256: "06c4678ff7b461dba8ed1f95c5e202ead3b57b70f549a22c17a94985121c0bf5" },
  "linux-arm64": { name: "pg0-linux-aarch64-gnu", sha256: "08bf6f5e04bc456fac2f8e76bfec5705c55f0b079f11b56efa5b1f6869e20f76" },
  "darwin-x64": { name: "pg0-darwin-x86_64", sha256: "2521f6344c29804b68e7074c05c335dfe826ef67b4a0185065750edc56ecd54e" },
  "darwin-arm64": { name: "pg0-darwin-aarch64", sha256: "29e9f0a2230d7c785af60482813fd5ee8d0c4e4c0561e9bdae3254e350252479" },
  "win32-x64": { name: "pg0-windows-x86_64.exe", sha256: "8894575679886fef41e176608da16f4448fdeea4cc60c02380422adc147260b7" },
};

const target = process.platform + "-" + process.arch;
const asset = binaries[target];
if (!asset) throw new Error("Unsupported pg0 target: " + target);
const directory = join(import.meta.dir, "..", "src-tauri", "binaries");
const dest = join(directory, process.platform === "win32" ? "pg0.exe" : "pg0");
const sha = (data: Uint8Array) => createHash("sha256").update(data).digest("hex");
await mkdir(directory, { recursive: true });

try {
  if (sha(await readFile(dest)) === asset.sha256) {
    console.log("Verified existing pg0 " + tag + " for " + target);
    process.exit(0);
  }
} catch { /* no existing bundle */ }

console.log("Fetching pinned pg0 binary (" + asset.name + ")");
const response = await fetch(release + asset.name, { redirect: "follow" });
if (!response.ok) throw new Error("Could not fetch pg0 release (" + response.status + ")");
const binary = new Uint8Array(await response.arrayBuffer());
if (sha(binary) !== asset.sha256) throw new Error("pg0 checksum mismatch; refusing to package");
const temporary = dest + ".partial";
try {
  await writeFile(temporary, binary, { mode: 0o755 });
  if (process.platform !== "win32") await chmod(temporary, 0o755);
  await rename(temporary, dest);
} finally {
  await rm(temporary, { force: true });
}
console.log("Verified and staged pg0 " + tag + " in Tauri resources");
