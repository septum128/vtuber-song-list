import type { ReactNode } from "react";
import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect } from "react";
import { Navbar } from "@/components/Navbar/Navbar";
import { AlertList } from "@/components/Common/Alert";
import { Loading } from "@/components/Common/Loading";
import { useAuth } from "@/hooks/useAuth";
import { UserKind } from "@/resources/enums";

type Props = {
  children: ReactNode;
};

const NAV_ITEMS = [
  { href: "/admin/song_diffs", label: "修正承認" },
  { href: "/admin/channels", label: "チャンネル管理" },
  { href: "/admin/videos", label: "動画管理" },
];

export function AdminLayout({ children }: Props) {
  const { user, isLoading } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!isLoading && (!user || user.kind !== UserKind.ADMIN)) {
      void router.replace("/");
    }
  }, [user, isLoading, router]);

  if (isLoading) return <Loading />;
  if (!user || user.kind !== UserKind.ADMIN) return null;

  return (
    <>
      <Navbar />
      <AlertList />
      <div className="container py-4">
        <div className="row">
          <div className="col-12 col-md-2 mb-3">
            <div className="list-group">
              {NAV_ITEMS.map((item) => (
                <Link
                  key={item.href}
                  href={item.href}
                  className={`list-group-item list-group-item-action small ${
                    router.pathname.startsWith(item.href) ? "active" : ""
                  }`}
                >
                  {item.label}
                </Link>
              ))}
            </div>
          </div>
          <div className="col-12 col-md-10">{children}</div>
        </div>
      </div>
    </>
  );
}
