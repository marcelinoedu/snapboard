import { motion, Variants, AnimatePresence } from "framer-motion";
import { useOverlay } from "../../contexts/overlayContext";
import { invoke } from "@tauri-apps/api/core";
import styles from "./overlay.module.css";

type OverlayViewProps = {
  children: React.ReactNode;
  className?: string;
};
const panelVariants: Variants = {
  hidden: { y: 40, opacity: 0 },
  visible: {
    y: 0,
    opacity: 1,
    transition: {
      duration: 0.25,
      ease: [0.25, 1, 0.36, 1] as const,
    },
  },
  exit: {
    y: 40,
    opacity: 0,
    transition: {
      duration: 0.25,
      ease: [0.25, 0, 1, 1] as const,
    },
  },
};

export function OverlayView({ children, className = "" }: OverlayViewProps) {
  const { visible } = useOverlay();

  return (
    <AnimatePresence
      mode="wait"
      onExitComplete={() => {
        invoke("finalize_overlay_close");
      }}
    >
      {visible && (
        <motion.div
          key="overlay"
          className={`${styles["overlay-root"]} ${className}`}
          variants={panelVariants}
          initial="hidden"
          animate="visible"
          exit="exit"
        >
          {children}
        </motion.div>
      )}
    </AnimatePresence>
  );
}
