import { describe, expect, it } from "vitest"
import {
  Option,
  Pattern,
  StandardGraph,
  Subject,
  Value,
  findFirst,
  fold,
  pipe,
  values,
} from "../src/index.js"

describe("@relateby/pattern", () => {
  it("builds native Subject values immutably", () => {
    const alice = Subject.fromId("alice")
      .withLabel("Person")
      .withProperty("name", Value.String({ value: "Alice" }))

    expect(alice.identity).toBe("alice")
    expect(alice.labels).toContain("Person")
    expect(Object.entries(alice.properties)).toContainEqual([
      "name",
      Value.String({ value: "Alice" }),
    ])
    expect(
      alice.equals(
        Subject.fromId("alice")
          .withLabel("Person")
          .withProperty("name", Value.String({ value: "Alice" }))
      )
    ).toBe(true)
  })

  it("supports native Pattern metrics and operations", () => {
    const tree = new Pattern({
      value: "root",
      elements: [
        Pattern.point("left"),
        new Pattern({ value: "right", elements: [Pattern.point("leaf")] }),
      ],
    })

    expect(tree.isAtomic).toBe(false)
    expect(tree.length).toBe(2)
    expect(tree.size).toBe(4)
    expect(tree.depth).toBe(2)
    expect(values(tree)).toEqual(["root", "left", "right", "leaf"])
    expect(pipe(tree, fold(0, (acc, value) => acc + value.length))).toBe(17)
    expect(pipe(tree, findFirst((value) => value.startsWith("lea")))).toEqual(Option.some("leaf"))
  })

  it("classifies native patterns with StandardGraph", () => {
    const alice = Pattern.point(Subject.fromId("alice").withLabel("Person"))
    const bob = Pattern.point(Subject.fromId("bob").withLabel("Person"))
    const knows = new Pattern({
      value: Subject.fromId("r1").withLabel("KNOWS"),
      elements: [alice, bob],
    })

    const graph = StandardGraph.fromPatterns([knows])

    expect(graph.nodeCount).toBe(2)
    expect(graph.relationshipCount).toBe(1)
    expect(Option.getOrUndefined(graph.node("alice"))?.value.identity).toBe("alice")
    expect(Option.getOrUndefined(graph.source("r1"))?.value.identity).toBe("alice")
    expect(Option.getOrUndefined(graph.target("r1"))?.value.identity).toBe("bob")

  })
})
