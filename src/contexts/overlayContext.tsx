import { createContext, useContext, useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { registerOverlayKeyboard } from "../utlis/keyboard";

type OverlayContextValue = {
  visible: boolean;
  requestClose: () => void;
};

const OverlayContext = createContext<OverlayContextValue | null>(null);

export function OverlayProvider({ children }: { children: React.ReactNode }) {

  registerOverlayKeyboard();
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    const open = listen("overlay:open", () => setVisible(true));
    const close = listen("overlay:close", () => setVisible(false));

    return () => {
      open.then((u) => u());
      close.then((u) => u());
    };
  }, []);

  const requestClose = () => {
    setVisible(false);
  };

  return (
    <OverlayContext.Provider value={{ visible, requestClose }}>
      {children}
    </OverlayContext.Provider>
  );
}


export function useOverlay() {
  const ctx = useContext(OverlayContext);
  if (!ctx) {
    throw new Error("useOverlay must be used inside OverlayProvider");
  }
  return ctx;
}
