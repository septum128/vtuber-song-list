import NextHead from "next/head";

type Props = {
  title?: string;
  description?: string;
  ogImage?: string;
  url?: string;
};

export const SITE_NAME = "VTuber Song List";
const DEFAULT_DESCRIPTION = "VTuberの歌枠セトリデータベース";

export function Head({
  title,
  description = DEFAULT_DESCRIPTION,
  ogImage,
  url,
}: Props) {
  const fullTitle = title ? `${title} | ${SITE_NAME}` : SITE_NAME;

  return (
    <NextHead>
      <title>{fullTitle}</title>
      <meta name="description" content={description} />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <link rel="icon" href="/favicon.ico" />
      {url && <meta property="og:url" content={url} />}
      <meta property="og:type" content="website" />
      <meta property="og:site_name" content={SITE_NAME} />
      <meta property="og:title" content={fullTitle} />
      <meta property="og:description" content={description} />
      {ogImage && <meta property="og:image" content={ogImage} />}
      <meta name="twitter:card" content="summary_large_image" />
      <meta name="twitter:title" content={fullTitle} />
      <meta name="twitter:description" content={description} />
      {ogImage && <meta name="twitter:image" content={ogImage} />}
    </NextHead>
  );
}
