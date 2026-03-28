import type { ReactNode } from "react";
import { Navbar } from "@/components/Navbar/Navbar";
import { AlertList } from "@/components/Common/Alert";

type Props = {
  children: ReactNode;
};

export function DefaultLayout({ children }: Props) {
  return (
    <>
      <Navbar />
      <AlertList />
      <main className="container py-4">{children}</main>
    </>
  );
}
