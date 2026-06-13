// schema.ts — JSON interchange format codec
//
// Converts between the JSON interchange format (RawPattern/RawSubject) and
// native Pattern<Subject> objects. The interchange format is defined by
// docs/pattern.schema.json in gram-data/tree-sitter-gram.

import { valueFromRaw } from "./value.js"
import { Subject } from "./subject.js"
import { Pattern } from "./pattern.js"
import type { Value } from "./value.js"

// --- Raw types (JSON interchange format) ---

export interface RawSubject {
  identity:   string
  labels:     ReadonlyArray<string>
  properties: Record<string, unknown>
}

export interface RawPattern {
  subject:  RawSubject
  elements: ReadonlyArray<RawPattern>
}

// --- Runtime validators ---

function isRecord(v: unknown): v is Record<string, unknown> {
  return typeof v === "object" && v !== null && !Array.isArray(v)
}

function isRawSubject(v: unknown): v is RawSubject {
  if (!isRecord(v)) return false
  if (typeof v["identity"] !== "string") return false
  if (!Array.isArray(v["labels"]) || !(v["labels"] as unknown[]).every(l => typeof l === "string")) return false
  if (!isRecord(v["properties"])) return false
  return true
}

function isRawPattern(v: unknown): v is RawPattern {
  if (!isRecord(v)) return false
  if (!isRawSubject(v["subject"])) return false
  if (!Array.isArray(v["elements"]) || !(v["elements"] as unknown[]).every(isRawPattern)) return false
  return true
}

export function validatePayload(raw: unknown): ReadonlyArray<RawPattern> {
  if (!Array.isArray(raw) || !(raw as unknown[]).every(isRawPattern)) {
    throw new TypeError("Invalid pattern payload from gram codec")
  }
  return raw as ReadonlyArray<RawPattern>
}

// --- Raw JSON → native ---

export function patternFromRaw(raw: RawPattern): Pattern<Subject> {
  const subject = Subject.from({
    identity: raw.subject.identity,
    labels:   raw.subject.labels,
    properties: Object.fromEntries(
      Object.entries(raw.subject.properties).map(([k, v]) => [k, valueFromRaw(v)])
    ),
  })
  return new Pattern({ value: subject, elements: raw.elements.map(patternFromRaw) })
}

// --- Native → raw JSON ---

export function patternToRaw(p: Pattern<Subject>): RawPattern {
  return {
    subject: {
      identity:   p.value.identity,
      labels:     [...p.value.labels].sort(),
      properties: Object.fromEntries(
        Object.entries(p.value.properties).map(([k, v]) => [k, valueToRaw(v)])
      ),
    },
    elements: p.elements.map(patternToRaw),
  }
}

function valueToRaw(v: Value): unknown {
  switch (v._tag) {
    case "StringVal":       return v.value
    case "IntVal":          return v.value
    case "FloatVal":        return v.value
    case "BoolVal":         return v.value
    case "NullVal":         throw new Error("JSON null is not representable as a gram value")
    case "SymbolVal":       return { type: "symbol",      value: v.value }
    case "TaggedStringVal": return { type: "tagged",      tag: v.tag, content: v.content }
    case "RangeVal":        return { type: "range",       lower: v.lower, upper: v.upper }
    case "MeasurementVal":  return { type: "measurement", unit: v.unit, value: v.value }
    case "ArrayVal":        return v.items.map(valueToRaw)
    case "MapVal":          return Object.fromEntries(
      Object.entries(v.entries).map(([k, val]) => [k, valueToRaw(val)])
    )
  }
}
