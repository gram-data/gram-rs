import { describe, expect, it } from "vitest"
import * as PublicApi from "../../src/index.js"

const expectedTopLevelExports = [
  "GramParseError",
  "Pattern",
  "StandardGraph",
  "Subject",
  "Value",
  "findFirst",
  "fold",
  "toGraphView",
  // New ops from 047-ts-py-parity:
  "anyValue",
  "allValues",
  "matches",
  "contains",
  "para",
  "unfold",
  "combine",
  "depthAt",
  "sizeAt",
  "indicesAt",
] as const

describe("@relateby/pattern public export inventory", () => {
  it("exposes the documented native top-level symbols", () => {
    for (const symbol of expectedTopLevelExports) {
      expect(PublicApi, `missing export: ${symbol}`).toHaveProperty(symbol)
    }
  })

  it("exposes JSON interchange codec functions", () => {
    expect(typeof PublicApi.patternFromRaw).toBe("function")
    expect(typeof PublicApi.patternToRaw).toBe("function")
    expect(typeof PublicApi.validatePayload).toBe("function")
  })

  it("exposes Pattern.pattern and Pattern.fromList static constructors", () => {
    expect(typeof PublicApi.Pattern.pattern).toBe("function")
    expect(typeof PublicApi.Pattern.fromList).toBe("function")
  })
})
