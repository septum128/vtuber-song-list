import NextHead from "next/head";

type Props = {
  title?: string;
  description?: string;
};

export const SITE_NAME = "VTuber Song List";
const DEFAULT_DESCRIPTION = "VTuberの歌枠セトリデータベース";

export function Head({ title, description = DEFAULT_DESCRIPTION }: Props) {
  const fullTitle = title ? `${title} | ${SITE_NAME}` : SITE_NAME;

  return (
    <NextHead>
      <title>{fullTitle}</title>
      <meta name="description" content={description} />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <link rel="icon" href="/favicon.ico" />
    </NextHead>
  );
}
