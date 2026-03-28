import { SITE_NAME } from "@/components/Common/Head";

export function Footer() {
  return (
    <footer className="bg-body-tertiary border-top py-3 mt-auto">
      <div className="container text-center text-muted small">
        © {new Date().getFullYear()} {SITE_NAME}
      </div>
    </footer>
  );
}
