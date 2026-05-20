import styles from "./auth.module.css";
import { createFileRoute, Outlet } from "@tanstack/solid-router";

export const Route = createFileRoute("/auth")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <main class={styles.wallpaperShell}>
      <div class={styles.backdrop} />

      <div class="relative z-10 grid min-h-screen items-center gap-8 p-5 md:p-8 lg:grid-cols-2">
        <section class="max-w-[34rem] p-2 text-white md:p-6">
          <h1 class="mt-6 text-[clamp(3rem,7vw,5.75rem)] font-black uppercase leading-[0.95] tracking-[-0.04em] text-white [text-shadow:0_6px_20px_rgb(0_0_0_/_0.4)]">
            Pokemon Packer
          </h1>

          <p class="mt-4 max-w-[32rem] text-[1.05rem] leading-7 text-cyan-50/90">
            A playful collection hub for trainers who want their packs, cards, and progress all in
            one place.
          </p>

          <div class="mt-7 flex gap-4" aria-hidden="true">
            <span class={styles.pokeball} />
            <span class={styles.pokeball} />
            <span class={styles.pokeball} />
          </div>
        </section>

        <section class="w-full justify-self-end lg:max-w-[31rem]">
          <Outlet />
        </section>
      </div>
    </main>
  );
}
