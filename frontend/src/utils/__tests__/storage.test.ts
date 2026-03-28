import { getToken, setToken, removeToken } from "../storage";

describe("storage", () => {
  beforeEach(() => {
    localStorage.clear();
  });

  describe("getToken", () => {
    it("returns null when no token is stored", () => {
      expect(getToken()).toBeNull();
    });

    it("returns the stored token", () => {
      localStorage.setItem("auth_token", "test-token");
      expect(getToken()).toBe("test-token");
    });
  });

  describe("setToken", () => {
    it("stores the token in localStorage", () => {
      setToken("my-token");
      expect(localStorage.getItem("auth_token")).toBe("my-token");
    });
  });

  describe("removeToken", () => {
    it("removes the token from localStorage", () => {
      localStorage.setItem("auth_token", "test-token");
      removeToken();
      expect(localStorage.getItem("auth_token")).toBeNull();
    });

    it("does not throw when no token exists", () => {
      expect(() => removeToken()).not.toThrow();
    });
  });
});
