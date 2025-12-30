import { emit } from "@tauri-apps/api/event";

export function registerOverlayKeyboard() {
  const onKey = (e: KeyboardEvent) => {
    if (e.key === "Escape") {
      emit("overlay:close");
    }
  };

  window.addEventListener("keydown", onKey);

  return () => {
    window.removeEventListener("keydown", onKey);
  };
}

