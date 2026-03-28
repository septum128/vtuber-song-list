import Link from "next/link";
import { useRouter } from "next/router";
import { useAuth } from "@/hooks/useAuth";
import { useTheme } from "@/context/ThemeProvider";
import { useAlerts } from "@/context/AlertsProvider";
import { UserKind } from "@/resources/enums";
import styles from "./Navbar.module.scss";

export function Navbar() {
  const { user, logout } = useAuth();
  const { theme, toggleTheme } = useTheme();
  const { addAlert } = useAlerts();
  const router = useRouter();

  async function handleLogout() {
    await logout();
    addAlert("success", "ログアウトしました");
    await router.push("/");
  }

  return (
    <nav className="navbar navbar-expand-lg bg-body-tertiary">
      <div className="container">
        <Link href="/" className={`navbar-brand ${styles.brand}`}>
          VTuber Song List
        </Link>

        <button
          className="navbar-toggler"
          type="button"
          data-bs-toggle="collapse"
          data-bs-target="#navbarMain"
          aria-controls="navbarMain"
          aria-expanded="false"
          aria-label="Toggle navigation"
        >
          <span className="navbar-toggler-icon" />
        </button>

        <div className="collapse navbar-collapse" id="navbarMain">
          <ul className="navbar-nav me-auto mb-2 mb-lg-0">
            <li className="nav-item">
              <Link
                href="/channels"
                className={`nav-link ${router.pathname.startsWith("/channels") ? "active" : ""}`}
              >
                チャンネル
              </Link>
            </li>
            {user && (
              <li className="nav-item">
                <Link
                  href="/favorites"
                  className={`nav-link ${router.pathname === "/favorites" ? "active" : ""}`}
                >
                  お気に入り
                </Link>
              </li>
            )}
            {user?.kind === UserKind.ADMIN && (
              <li className="nav-item">
                <Link
                  href="/admin"
                  className={`nav-link ${router.pathname.startsWith("/admin") ? "active" : ""}`}
                >
                  管理
                </Link>
              </li>
            )}
          </ul>

          <div className="d-flex align-items-center gap-2">
            <button
              type="button"
              className={`btn btn-sm btn-outline-secondary ${styles.themeBtn}`}
              onClick={toggleTheme}
              aria-label="テーマ切替"
              title={theme === "light" ? "ダークモードに切替" : "ライトモードに切替"}
            >
              {theme === "light" ? "🌙" : "☀️"}
            </button>

            {user ? (
              <>
                <span className="navbar-text small">{user.name}</span>
                <button
                  type="button"
                  className="btn btn-sm btn-outline-secondary"
                  onClick={handleLogout}
                >
                  ログアウト
                </button>
              </>
            ) : (
              <>
                <Link
                  href="/session/new"
                  className="btn btn-sm btn-outline-primary"
                >
                  ログイン
                </Link>
                <Link href="/user/new" className="btn btn-sm btn-primary">
                  登録
                </Link>
              </>
            )}
          </div>
        </div>
      </div>
    </nav>
  );
}
