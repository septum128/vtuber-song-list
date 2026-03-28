import { useState } from "react";
import { useToggleFavorite } from "@/hooks/useFavorites";

type Props = {
  songItemId: number;
  favorited: boolean;
};

export function FavoriteButton({ songItemId, favorited }: Props) {
  const { toggle } = useToggleFavorite();
  const [busy, setBusy] = useState(false);

  async function handleClick() {
    if (busy) return;
    setBusy(true);
    try {
      await toggle(songItemId, favorited);
    } finally {
      setBusy(false);
    }
  }

  return (
    <button
      type="button"
      className={`btn btn-sm border-0 p-0 lh-1 ${favorited ? "text-danger" : "text-body-tertiary"}`}
      onClick={handleClick}
      disabled={busy}
      aria-label={favorited ? "お気に入りを解除" : "お気に入りに追加"}
      title={favorited ? "お気に入りを解除" : "お気に入りに追加"}
      style={{ fontSize: "1.1rem" }}
    >
      {favorited ? "♥" : "♡"}
    </button>
  );
}
