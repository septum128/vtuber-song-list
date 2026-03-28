import type { ReactNode } from "react";
import { Navbar } from "@/components/Navbar/Navbar";
import { AlertList } from "@/components/Common/Alert";
import { Footer } from "@/components/Common/Footer";

type Props = {
  children: ReactNode;
};

export function DefaultLayout({ children }: Props) {
  return (
    <div className="d-flex flex-column min-vh-100">
      <Navbar />
      <AlertList />
      <main className="container py-4 flex-grow-1">{children}</main>
      <Footer />
    </div>
  );
}
