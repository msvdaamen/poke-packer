import { Link, createFileRoute } from "@tanstack/solid-router";
import styles from "./auth.module.css";

export const Route = createFileRoute("/auth/sign-in")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <div class="relative rounded-[2rem] border border-white/15 bg-slate-950/76 p-6 shadow-[0_20px_60px_rgb(15_23_42_/_0.45)] backdrop-blur-xl sm:p-8">
      <div class={styles.cardAccent} aria-hidden="true" />

      <p class="text-[0.8rem] font-extrabold uppercase tracking-[0.14em] text-red-300">
        Trainer Login
      </p>
      <h2 class="mt-3 max-w-[20rem] text-[2.2rem] leading-[1.05] font-black text-slate-50 sm:text-[2.35rem]">
        Jump back into your Poke Pack journey
      </h2>
      <p class="mt-3.5 max-w-[24rem] leading-7 text-slate-300">
        Pick up where you left off, organize your collection, and head back into your Pokemon
        adventure.
      </p>

      <form class="mt-6 grid gap-4">
        <label class="grid gap-2 text-[0.92rem] font-bold text-slate-100">
          <span>Email</span>
          <input
            class="w-full rounded-2xl border border-white/12 bg-slate-900/92 px-4 py-4 text-slate-100 outline-none transition duration-150 placeholder:text-slate-500 focus:-translate-y-px focus:border-amber-300 focus:shadow-[0_0_0_4px_rgb(251_191_36_/_0.15)]"
            type="email"
            name="email"
            placeholder="trainer@pokepack.dev"
          />
        </label>

        <label class="grid gap-2 text-[0.92rem] font-bold text-slate-100">
          <span>Password</span>
          <input
            class="w-full rounded-2xl border border-white/12 bg-slate-900/92 px-4 py-4 text-slate-100 outline-none transition duration-150 placeholder:text-slate-500 focus:-translate-y-px focus:border-amber-300 focus:shadow-[0_0_0_4px_rgb(251_191_36_/_0.15)]"
            type="password"
            name="password"
            placeholder="Enter your password"
          />
        </label>

        <button
          class="mt-2 grid cursor-pointer grid-cols-[1fr_auto] items-center rounded-[1.4rem] border border-red-300/35 bg-linear-to-r from-slate-950 via-slate-900 to-red-950 px-5 py-3 text-left shadow-[0_14px_30px_rgb(2_6_23_/_0.45)] transition duration-150 hover:-translate-y-0.5 hover:border-amber-300/50 hover:shadow-[0_20px_36px_rgb(124_45_18_/_0.35)]"
          type="submit"
        >
          <span>
            <span class="block text-[0.72rem] font-bold uppercase tracking-[0.22em] text-red-200/80">
              Trainer Access
            </span>
            <span class="block text-lg font-black uppercase tracking-[0.08em] text-amber-200">
              Sign In
            </span>
          </span>
          <span class="ml-4 flex h-12 w-12 items-center justify-center rounded-full border-4 border-slate-950 bg-[linear-gradient(to_bottom,_#ef4444_0_46%,_#111827_46_54%,_#ffffff_54_100%)] shadow-[inset_0_2px_8px_rgb(255_255_255_/_0.22)]">
            <span class="h-4 w-4 rounded-full border-2 border-slate-950 bg-white" />
          </span>
        </button>
      </form>

      <p class="mt-5 text-slate-300">
        New trainer?{" "}
        <Link
          class="font-extrabold text-red-300 no-underline hover:text-amber-200 hover:underline"
          to="/auth/sign-up"
        >
          Create an account
        </Link>
      </p>
    </div>
  );
}
