import { apiFetch, buildQuery } from "../api";

// Mock storage to control token
jest.mock("../storage", () => ({
  getToken: jest.fn(),
}));
import { getToken } from "../storage";
const mockGetToken = getToken as jest.MockedFunction<typeof getToken>;

describe("apiFetch", () => {
  beforeEach(() => {
    jest.resetAllMocks();
    mockGetToken.mockReturnValue(null);
  });

  it("returns parsed JSON on success", async () => {
    global.fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ id: 1, name: "test" }),
    } as Response);

    const result = await apiFetch<{ id: number; name: string }>("/api/test");
    expect(result).toEqual({ id: 1, name: "test" });
  });

  it("throws with error message on non-ok response", async () => {
    global.fetch = jest.fn().mockResolvedValue({
      ok: false,
      status: 400,
      json: async () => ({ message: "Bad request" }),
    } as Response);

    await expect(apiFetch("/api/test")).rejects.toThrow("Bad request");
  });

  it("throws with HTTP status fallback when error body is not JSON", async () => {
    global.fetch = jest.fn().mockResolvedValue({
      ok: false,
      status: 500,
      json: async () => { throw new Error("not json"); },
    } as unknown as Response);

    await expect(apiFetch("/api/test")).rejects.toThrow("HTTP 500");
  });

  it("does not add Authorization header when auth is false", async () => {
    mockGetToken.mockReturnValue("my-token");
    global.fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: async () => ({}),
    } as Response);

    await apiFetch("/api/test", { auth: false });

    const [, options] = (global.fetch as jest.Mock).mock.calls[0] as [string, RequestInit];
    expect((options.headers as Record<string, string>)["Authorization"]).toBeUndefined();
  });

  it("adds Authorization header when auth is true and token exists", async () => {
    mockGetToken.mockReturnValue("my-token");
    global.fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: async () => ({}),
    } as Response);

    await apiFetch("/api/test", { auth: true });

    const [, options] = (global.fetch as jest.Mock).mock.calls[0] as [string, RequestInit];
    expect((options.headers as Record<string, string>)["Authorization"]).toBe("Bearer my-token");
  });

  it("does not add Authorization header when auth is true but no token", async () => {
    mockGetToken.mockReturnValue(null);
    global.fetch = jest.fn().mockResolvedValue({
      ok: true,
      json: async () => ({}),
    } as Response);

    await apiFetch("/api/test", { auth: true });

    const [, options] = (global.fetch as jest.Mock).mock.calls[0] as [string, RequestInit];
    expect((options.headers as Record<string, string>)["Authorization"]).toBeUndefined();
  });
});

describe("buildQuery", () => {
  it("returns empty string for empty params", () => {
    expect(buildQuery({})).toBe("");
  });

  it("returns empty string when all values are null/undefined/empty", () => {
    expect(buildQuery({ a: null, b: undefined, c: "" })).toBe("");
  });

  it("builds query string from valid params", () => {
    const result = buildQuery({ page: 2, count: 30 });
    expect(result).toBe("?page=2&count=30");
  });

  it("excludes null and undefined values", () => {
    const result = buildQuery({ q: "hello", since: null, until: undefined });
    expect(result).toBe("?q=hello");
  });

  it("includes boolean false as a value", () => {
    const result = buildQuery({ published: false });
    expect(result).toBe("?published=false");
  });
});
