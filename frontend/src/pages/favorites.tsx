import { useEffect } from "react";
import { useRouter } from "next/router";
import { Head } from "@/components/Common/Head";
import { Loading } from "@/components/Common/Loading";
import { DefaultLayout } from "@/layouts/DefaultLayout";
import { FavoriteList } from "@/containers/Favorites/FavoriteList";
import { useAuth } from "@/hooks/useAuth";

export default function FavoritesPage() {
  const { user, isLoading } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!isLoading && !user) {
      void router.replace("/session/new");
    }
  }, [user, isLoading, router]);

  if (isLoading || !user) {
    return (
      <DefaultLayout>
        <Head title="お気に入り" />
        <Loading />
      </DefaultLayout>
    );
  }

  return (
    <DefaultLayout>
      <Head title="お気に入り" />
      <h1 className="h3 mb-4">お気に入り</h1>
      <FavoriteList />
    </DefaultLayout>
  );
}
