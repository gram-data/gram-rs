# `@relateby/pattern`

Native TypeScript `Pattern`, `Subject`, and `StandardGraph` APIs backed by the Rust gram codec's JSON interchange layer.

## Install

```bash
npm install @relateby/pattern effect
```

## Quick Start

```typescript
import { Effect, Equal, Option, pipe } from "effect"
import { Gram, Pattern, StandardGraph, Subject, Value, findFirst, fold } from "@relateby/pattern"

const alice = Subject.fromId("alice")
  .withLabel("Person")
  .withProperty("name", Value.String({ value: "Alice" }))
const bob = Subject.fromId("bob").withLabel("Person")

const relationship = new Pattern({
  value: Subject.fromId("r1").withLabel("KNOWS"),
  elements: [Pattern.point(alice), Pattern.point(bob)],
})

const graph = StandardGraph.fromPatterns([relationship])

// Parse gram notation and build a graph in two explicit steps:
const parsed = await Effect.runPromise(Gram.parse("(alice:Person)-->(bob:Person)"))
const parsedGraph = StandardGraph.fromPatterns(parsed)

console.log(graph.nodeCount)
console.log(Option.getOrUndefined(graph.node("alice"))?.value.identity)
console.log(pipe(relationship, fold(0, (acc) => acc + 1)))
console.log(pipe(relationship, findFirst((subject) => subject.identity === "bob")))
console.log(Equal.equals(parsed[0]?.value, alice))
```

## Gram Effects

`Gram.parse`, `Gram.stringify`, and `Gram.validate` return `Effect` values:

```typescript
import { Effect, pipe } from "effect"
import { Gram } from "@relateby/pattern"

const rendered = await pipe(
  Gram.parse("(alice:Person)"),
  Effect.flatMap((patterns) => Gram.stringify(patterns)),
  Effect.runPromise
)
```

Use `Option.getOrUndefined()` for lookups and `Equal.equals()` for structural equality.

## JSON Interchange

`Pattern<Subject>` can be serialized to and from a plain JSON-compatible format defined by the [gram interchange schema](https://github.com/gram-data/tree-sitter-gram/blob/main/docs/pattern.schema.json). This is useful when you need to pass patterns across a network boundary — for example, from a Next.js server component (where WASM is available) to a client component (where it is not).

**Producer** (has WASM, e.g. a server):

```typescript
import { Gram } from "@relateby/gram"

// Parse gram and return the raw interchange objects directly —
// no Pattern<Subject> construction overhead, ready for JSON.stringify.
const raw = await Gram.parseRaw("(alice:Person)-[:KNOWS]->(bob:Person)")
res.json(raw)
```

**Consumer** (no WASM needed, e.g. a browser client):

```typescript
import { patternFromRaw, validatePayload } from "@relateby/pattern"

// Validate the incoming payload shape and reconstruct native patterns.
const patterns = validatePayload(data).map(patternFromRaw)
```

**Round-trip** (native ↔ JSON):

```typescript
import { patternFromRaw, patternToRaw } from "@relateby/pattern"

const raw = patternToRaw(pattern)          // Pattern<Subject> → RawPattern
const restored = patternFromRaw(raw)       // RawPattern → Pattern<Subject>
```

The `RawPattern` and `RawSubject` types are exported for use in TypeScript type annotations.
