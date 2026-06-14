export { Gram } from "./gram-browser.js"
export { GramParseError } from "@relateby/pattern"

/**
 * Warm the shared WASM-backed Gram surface for callers that prefer explicit init.
 * Gram loads lazily on first use; call this at app startup to pay that cost upfront.
 */
export async function init(): Promise<void> {
  const { Gram } = await import("./gram-browser.js")
  await Gram.validate("(init)")
}
