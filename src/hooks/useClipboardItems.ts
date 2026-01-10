import { useEffect, useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  ClipboardItem,
  ClipboardItemSchema,
  ClipboardItemArraySchema,
} from "../types/clipboardItem";

export function useClipboardItems() {
  const [clipboard_history, setClipboardHistory] = useState<ClipboardItem[]>(
    []
  );
  const [current_clipboard_item, setCurrentClipboardItem] =
    useState<ClipboardItem | null>(null);

  const [activeKinds, setActiveKinds] = useState<string[]>([]);

  const deleteItem = async (id: string) => {
    await invoke("delete_item", { itemId: id });
    setClipboardHistory((prev) => prev.filter((item) => item.id !== id));
  };

  const selectItemToClipboard = async (content: string) => {
    await invoke("select_item_to_clipboard", { content: content });
    return clipboard_history.find((item) => item.content === content) ?? null;
  };

  const reLoadItems = async () => {
    const data = await invoke("list_clipboard_history");
    const parsed = ClipboardItemArraySchema.parse(data);

    setClipboardHistory(parsed);
  };

  const deleteAllItems = async () => {
    await invoke("clear_all");
    setClipboardHistory([]);
  };

  const stopApplication = async () => {
    await invoke("stop_app");
  };

  const filteredClipboardHistory = useMemo(() => {
    if (activeKinds.length === 0) return clipboard_history;

    return clipboard_history.filter((item) => activeKinds.includes(item.kind));
  }, [clipboard_history, activeKinds]);

  const toggleKindFilter = (kind: string) => {
    setActiveKinds((prev) =>
      prev.includes(kind) ? prev.filter((k) => k !== kind) : [...prev, kind]
    );
  };

  const clearFilters = () => setActiveKinds([]);



  useEffect(() => {
    invoke("list_clipboard_history").then((data) => {
      console.log("Data from list_clipboard_history:", data);
      const parsed = ClipboardItemArraySchema.parse(data);
      setClipboardHistory(parsed);
    });

    invoke("get_current_clipboard_item").then((data) => {
      console.log("Data from get_current_clipboard_item:", data);
      const parsed = ClipboardItemSchema.parse(data);
      setCurrentClipboardItem(parsed);
    });
  }, []);

  useEffect(() => {
    const unlisten = listen("clipboard:new", (e) => {
      const parsed = ClipboardItemSchema.safeParse(e.payload);
      if (!parsed.success) return;

      setClipboardHistory((prev) => {
        if (prev.some((i) => i.id === parsed.data.id)) return prev;
        return [parsed.data, ...prev];
      });

      setCurrentClipboardItem(parsed.data);
    });

    return () => {
      unlisten.then((u) => u());
    };
  }, []);

  useEffect(() => {
    const unlisten = listen("clipboard:change", (e) => {
      const parsed = ClipboardItemSchema.safeParse(e.payload);
      if (!parsed.success) return;
      setCurrentClipboardItem(parsed.data);
    });

    return () => {
      unlisten.then((u) => u());
    };
  }, []);

  return {
    clipboard_history: filteredClipboardHistory,
    current_clipboard_item,
    deleteItem,
    deleteAllItems,
    reLoadItems,
    selectItemToClipboard,
    stopApplication,
    activeKinds,
    toggleKindFilter,
    clearFilters,
  };
}
