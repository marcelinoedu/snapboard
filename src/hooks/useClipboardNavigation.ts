import { useEffect, useRef, useState } from "react";
import { ClipboardItem } from "../types/clipboardItem";


export function useClipboardNavigation(items: ClipboardItem[]) {
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);
  const [hasNavigated, setHasNavigated] = useState(false);


  const [itemToSelect, setItemToSelect] = useState<ClipboardItem | null>(null);
  const [itemToDelete, setItemToDelete] = useState<ClipboardItem | null>(null);
  const [itemToPreview, setItemToPreview] = useState<ClipboardItem | null>(null);

  const containerRef = useRef<HTMLDivElement | null>(null);

  /* ────────────────────────────────
   * Mantém índice válido
   * ──────────────────────────────── */
  useEffect(() => {
    if (items.length === 0) {
      setSelectedIndex(null);
      return;
    }

    if (selectedIndex === null) return;

    if (selectedIndex >= items.length) {
      setSelectedIndex(items.length - 1);
    }
  }, [items.length, selectedIndex]);

  /* ────────────────────────────────
   * Hotkeys
   * ──────────────────────────────── */
  useEffect(() => {
    function onKeyDown(e: KeyboardEvent) {
      if (items.length === 0) return;

      // navegação
      if (e.key === "ArrowRight") {
        e.preventDefault();
        setHasNavigated(true);
        setSelectedIndex((i) =>
          Math.min((i ?? 0) + 1, items.length - 1)
        );
      }

      if (e.key === "ArrowLeft") {
        e.preventDefault();
        setHasNavigated(true);
        setSelectedIndex((i) =>
          Math.max((i ?? 0) - 1, 0)
        );
      }

      // preview (Space)
      if (e.key === " " || e.key === "Spacebar") {
        if (selectedIndex !== null) {
          e.preventDefault();
          setItemToPreview(items[selectedIndex]);
        }
      }

      // copy (Enter)
      if (e.key === "Enter") {
        if (selectedIndex !== null) {
          e.preventDefault();
          setItemToSelect(items[selectedIndex]);
        }
      }

      // delete
      if (e.key === "Delete" || e.key === "Backspace") {
        if (selectedIndex !== null) {
          e.preventDefault();
          setItemToDelete(items[selectedIndex]);
        }
      }
    }

    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [items, selectedIndex]);

  /* ────────────────────────────────
   * Scroll automático
   * ──────────────────────────────── */
  useEffect(() => {
    if (selectedIndex === null) return;

    const container = containerRef.current;
    if (!container) return;

    const child = container.children[selectedIndex] as HTMLElement | undefined;
    if (!child) return;

    child.scrollIntoView({
      behavior: "smooth",
      inline: "center",
      block: "nearest",
    });

    setHasNavigated(true);
    
  }, [selectedIndex]);

  /* ────────────────────────────────
   * Mensagem de hotkeys
   * ──────────────────────────────── */
  const navMessage =
    items.length === 0
      ? null
      : hasNavigated
      ? "Press Space to preview"
      : null;

  /* ────────────────────────────────
   * Callbacks para o App confirmar ações
   * ──────────────────────────────── */
  const clearItemToSelect = () => setItemToSelect(null);
  const clearItemToDelete = () => setItemToDelete(null);
  const clearItemToPreview = () => setItemToPreview(null);

  return {
    // seleção
    selectedIndex,
    selectedItem:
      selectedIndex !== null ? items[selectedIndex] : null,

    // refs
    containerRef,

    // mensagens
    navMessage,

    // intenções
    itemToSelect,
    itemToDelete,
    itemToPreview,

    // confirmação de ação (App → hook)
    clearItemToSelect,
    clearItemToDelete,
    clearItemToPreview,

    // controle manual (opcional)
    setSelectedIndex,
  };
}
