#!/usr/bin/env node
// TypeScript interop check: read gram from stdin, write JSON interchange to stdout.
// Used by tests/interop/run.sh — must produce identical JSON to rs_interop and py_interop.

import { createReadStream } from "node:fs"
import { createInterface } from "node:readline"
import { Gram } from "../../typescript/packages/pattern/dist/gram.js"

const lines = []
const rl = createInterface({ input: process.stdin })
rl.on("line", line => {
  if (!line.trimStart().startsWith("//")) lines.push(line)
})
rl.on("close", async () => {
  const input = lines.join("\n")
  try {
    const raw = await Gram.parseRaw(input)
    console.log(JSON.stringify(raw))
  } catch (err) {
    process.stderr.write(`parse error: ${err.message}\n`)
    process.exit(1)
  }
})
