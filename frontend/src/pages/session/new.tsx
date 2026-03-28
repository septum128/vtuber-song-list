import { useEffect } from "react";
import { useRouter } from "next/router";
import { Head } from "@/components/Common/Head";
import { Loading } from "@/components/Common/Loading";
import { DefaultLayout } from "@/layouts/DefaultLayout";
import { LoginForm } from "@/containers/Session/LoginForm";
import { useAuth } from "@/hooks/useAuth";

export default function LoginPage() {
  const { user, isLoading } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!isLoading && user) {
      void router.replace("/");
    }
  }, [user, isLoading, router]);

  if (isLoading || user) {
    return (
      <DefaultLayout>
        <Head title="ログイン" />
        <Loading />
      </DefaultLayout>
    );
  }

  return (
    <DefaultLayout>
      <Head title="ログイン" />
      <div className="row justify-content-center">
        <div className="col-12 col-sm-8 col-md-6 col-lg-4">
          <h1 className="h4 mb-4 text-center">ログイン</h1>
          <LoginForm />
        </div>
      </div>
    </DefaultLayout>
  );
}
