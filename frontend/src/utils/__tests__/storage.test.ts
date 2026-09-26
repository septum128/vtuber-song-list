import {
  getToken,
  setToken,
  removeToken,
  getAdminVideoListPerPage,
  setAdminVideoListPerPage,
  getAdminVideoListOnlySongLives,
  setAdminVideoListOnlySongLives,
} from "../storage";

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

  describe("getAdminVideoListPerPage", () => {
    it("returns null when nothing is stored", () => {
      expect(getAdminVideoListPerPage()).toBeNull();
    });

    it("returns the stored value as a number", () => {
      localStorage.setItem("admin_video_list_per_page", "50");
      expect(getAdminVideoListPerPage()).toBe(50);
    });
  });

  describe("setAdminVideoListPerPage", () => {
    it("stores the value in localStorage", () => {
      setAdminVideoListPerPage(100);
      expect(localStorage.getItem("admin_video_list_per_page")).toBe("100");
    });
  });

  describe("getAdminVideoListOnlySongLives", () => {
    it("returns false when nothing is stored", () => {
      expect(getAdminVideoListOnlySongLives()).toBe(false);
    });

    it("returns true when stored as true", () => {
      localStorage.setItem("admin_video_list_only_song_lives", "true");
      expect(getAdminVideoListOnlySongLives()).toBe(true);
    });

    it("returns false when stored as false", () => {
      localStorage.setItem("admin_video_list_only_song_lives", "false");
      expect(getAdminVideoListOnlySongLives()).toBe(false);
    });
  });

  describe("setAdminVideoListOnlySongLives", () => {
    it("stores true in localStorage", () => {
      setAdminVideoListOnlySongLives(true);
      expect(localStorage.getItem("admin_video_list_only_song_lives")).toBe("true");
    });

    it("stores false in localStorage", () => {
      setAdminVideoListOnlySongLives(false);
      expect(localStorage.getItem("admin_video_list_only_song_lives")).toBe("false");
    });
  });
});
