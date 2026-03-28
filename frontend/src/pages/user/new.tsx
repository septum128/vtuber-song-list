import { useEffect } from "react";
import { useRouter } from "next/router";
import { Head } from "@/components/Common/Head";
import { Loading } from "@/components/Common/Loading";
import { DefaultLayout } from "@/layouts/DefaultLayout";
import { RegisterForm } from "@/containers/User/RegisterForm";
import { useAuth } from "@/hooks/useAuth";

export default function RegisterPage() {
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
        <Head title="ユーザー登録" />
        <Loading />
      </DefaultLayout>
    );
  }

  return (
    <DefaultLayout>
      <Head title="ユーザー登録" />
      <div className="row justify-content-center">
        <div className="col-12 col-sm-8 col-md-6 col-lg-4">
          <h1 className="h4 mb-4 text-center">ユーザー登録</h1>
          <RegisterForm />
        </div>
      </div>
    </DefaultLayout>
  );
}
