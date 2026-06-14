// gram-browser.ts — browser/bundler entry point for the Gram WASM loader.
//
// This module is selected via the "browser" export condition in package.json.
// It contains NO Node.js built-in imports (no "module", "url", "path") so
// bundlers targeting browsers (Turbopack, webpack, Vite) can resolve it
// without hitting `Can't resolve 'fs'` or similar errors.

import { GramParseError, Pattern, Subject, validatePayload, patternFromRaw, patternToRaw } from "@relateby/pattern"
import type { RawPattern } from "@relateby/pattern"

interface WasmGram {
  parse(input: string): unknown
  stringify(patterns: unknown): string
  validate(input: string): string[]
  parseWithHeader(input: string): unknown
  stringifyWithHeader(input: unknown): string
}

let wasmGram: WasmGram | null = null

async function loadWasm(): Promise<WasmGram> {
  if (wasmGram !== null) return wasmGram

  try {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const mod = await import(/* @vite-ignore */ "./wasm/pattern_wasm.js" as string) as any
    // wasm-pack bundler-target modules export an async init() as the default
    // export. Call it explicitly to ensure the WASM binary is instantiated
    // in environments where the bundler did not auto-initialize it.
    if (typeof mod.default === "function") {
      await (mod.default as () => Promise<unknown>)()
    }
    if (mod.Gram) {
      wasmGram = mod.Gram as WasmGram
      return wasmGram
    }
  } catch {
    // fall through to unavailable stub
  }

  const unavailable = (): never => {
    throw new Error(
      "Gram WASM bindings are unavailable. " +
      "Ensure wasm/ is present and the bundler is configured to handle .wasm files."
    )
  }
  wasmGram = {
    parse: unavailable,
    stringify: unavailable,
    validate: () => unavailable(),
    parseWithHeader: unavailable,
    stringifyWithHeader: unavailable,
  }
  return wasmGram
}

export const Gram = {
  async parse(input: string): Promise<ReadonlyArray<Pattern<Subject>>> {
    try {
      const wasm = await loadWasm()
      const raw = wasm.parse(input)
      return validatePayload(raw).map(patternFromRaw)
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input, cause })
    }
  },

  async parseRaw(input: string): Promise<ReadonlyArray<RawPattern>> {
    try {
      const wasm = await loadWasm()
      return validatePayload(wasm.parse(input))
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input, cause })
    }
  },

  async stringify(patterns: ReadonlyArray<Pattern<Subject>>): Promise<string> {
    try {
      const wasm = await loadWasm()
      return wasm.stringify(patterns.map(patternToRaw))
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input: "(stringify)", cause })
    }
  },

  async validate(input: string): Promise<void> {
    try {
      const wasm = await loadWasm()
      const errors = wasm.validate(input) as string[]
      if (errors.length > 0) {
        throw new GramParseError({ input, cause: errors.join("; ") })
      }
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input, cause })
    }
  },

  async parseWithHeader(input: string): Promise<{
    header: Record<string, unknown> | undefined
    patterns: ReadonlyArray<Pattern<Subject>>
  }> {
    try {
      const wasm = await loadWasm()
      const result = wasm.parseWithHeader(input) as { header: Record<string, unknown> | null; patterns: unknown[] }
      const header = result.header == null ? undefined : result.header
      const patterns = validatePayload(result.patterns).map(patternFromRaw)
      return { header, patterns }
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input, cause })
    }
  },

  async stringifyWithHeader(
    header: Record<string, unknown> | undefined,
    patterns: ReadonlyArray<Pattern<Subject>>,
  ): Promise<string> {
    try {
      const wasm = await loadWasm()
      return wasm.stringifyWithHeader({ header: header ?? null, patterns: patterns.map(patternToRaw) })
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input: "(stringifyWithHeader)", cause })
    }
  },
}
