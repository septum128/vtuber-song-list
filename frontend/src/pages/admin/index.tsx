import { useEffect } from "react";
import { useRouter } from "next/router";

export default function AdminIndex() {
  const router = useRouter();
  useEffect(() => {
    void router.replace("/admin/song_diffs");
  }, [router]);
  return null;
}
