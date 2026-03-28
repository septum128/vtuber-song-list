import { useState } from "react";
import { useForm } from "react-hook-form";
import { useRouter } from "next/router";
import Link from "next/link";
import { useAuth } from "@/hooks/useAuth";
import { useAlerts } from "@/context/AlertsProvider";

type FormValues = {
  name: string;
  password: string;
  password_confirmation: string;
};

export function RegisterForm() {
  const { register: registerUser } = useAuth();
  const { addAlert } = useAlerts();
  const router = useRouter();
  const [submitting, setSubmitting] = useState(false);

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<FormValues>();

  const password = watch("password");

  async function onSubmit(values: FormValues) {
    setSubmitting(true);
    try {
      await registerUser(values.name, values.password, values.password_confirmation);
      addAlert("success", "登録しました");
      await router.push("/");
    } catch (e) {
      addAlert("danger", e instanceof Error ? e.message : "登録に失敗しました");
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} noValidate>
      <div className="mb-3">
        <label htmlFor="reg-name" className="form-label">
          ユーザー名
        </label>
        <input
          id="reg-name"
          type="text"
          className={`form-control ${errors.name ? "is-invalid" : ""}`}
          autoComplete="username"
          {...register("name", {
            required: "ユーザー名を入力してください",
            maxLength: { value: 255, message: "ユーザー名は255文字以内で入力してください" },
          })}
        />
        {errors.name && (
          <div className="invalid-feedback">{errors.name.message}</div>
        )}
      </div>

      <div className="mb-3">
        <label htmlFor="reg-password" className="form-label">
          パスワード
        </label>
        <input
          id="reg-password"
          type="password"
          className={`form-control ${errors.password ? "is-invalid" : ""}`}
          autoComplete="new-password"
          {...register("password", {
            required: "パスワードを入力してください",
            minLength: { value: 8, message: "パスワードは8文字以上で入力してください" },
          })}
        />
        {errors.password && (
          <div className="invalid-feedback">{errors.password.message}</div>
        )}
      </div>

      <div className="mb-4">
        <label htmlFor="reg-password-confirmation" className="form-label">
          パスワード（確認）
        </label>
        <input
          id="reg-password-confirmation"
          type="password"
          className={`form-control ${errors.password_confirmation ? "is-invalid" : ""}`}
          autoComplete="new-password"
          {...register("password_confirmation", {
            required: "確認用パスワードを入力してください",
            validate: (v) =>
              v === password || "パスワードと確認用パスワードが一致しません",
          })}
        />
        {errors.password_confirmation && (
          <div className="invalid-feedback">
            {errors.password_confirmation.message}
          </div>
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
            登録中...
          </>
        ) : (
          "登録する"
        )}
      </button>

      <p className="text-center mt-3 small">
        すでにアカウントをお持ちの方は{" "}
        <Link href="/session/new">こちらからログイン</Link>
      </p>
    </form>
  );
}
