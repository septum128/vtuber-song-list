import { useState } from "react";
import { useForm } from "react-hook-form";
import { useRouter } from "next/router";
import Link from "next/link";
import { useAuth } from "@/hooks/useAuth";
import { useAlerts } from "@/context/AlertsProvider";

type FormValues = {
  name: string;
  password: string;
};

export function LoginForm() {
  const { login } = useAuth();
  const { addAlert } = useAlerts();
  const router = useRouter();
  const [submitting, setSubmitting] = useState(false);

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<FormValues>();

  async function onSubmit(values: FormValues) {
    setSubmitting(true);
    try {
      await login(values.name, values.password);
      addAlert("success", "ログインしました");
      await router.push("/");
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "ログインに失敗しました");
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} noValidate>
      <div className="mb-3">
        <label htmlFor="login-name" className="form-label">
          ユーザー名
        </label>
        <input
          id="login-name"
          type="text"
          className={`form-control ${errors.name ? "is-invalid" : ""}`}
          autoComplete="username"
          {...register("name", { required: "ユーザー名を入力してください" })}
        />
        {errors.name && (
          <div className="invalid-feedback">{errors.name.message}</div>
        )}
      </div>

      <div className="mb-4">
        <label htmlFor="login-password" className="form-label">
          パスワード
        </label>
        <input
          id="login-password"
          type="password"
          className={`form-control ${errors.password ? "is-invalid" : ""}`}
          autoComplete="current-password"
          {...register("password", {
            required: "パスワードを入力してください",
            minLength: { value: 8, message: "パスワードは8文字以上で入力してください" },
          })}
        />
        {errors.password && (
          <div className="invalid-feedback">{errors.password.message}</div>
        )}
      </div>

      <button
        type="submit"
        className="btn btn-primary w-100"
        disabled={submitting}
      >
        {submitting ? (
          <>
            <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true" />
            ログイン中...
          </>
        ) : (
          "ログイン"
        )}
      </button>

      <p className="text-center mt-3 small">
        アカウントをお持ちでない方は{" "}
        <Link href="/user/new">こちらから登録</Link>
      </p>
    </form>
  );
}
