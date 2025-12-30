import { OverlayView } from "./components/overlay/overlayView";
import { useClipboardItems } from "./hooks/useClipboardItems";
import { ClipboardCardSkeleton } from "./components/clipboard/clipboardCardSkeleton";
import { ClipboardCard } from "./components/clipboard/clipboardCard";
import { useClipboardNavigation } from "./hooks/useClipboardNavigation";
import { useEffect } from "react";
import { emit } from "@tauri-apps/api/event";
import { KindFilter } from "./components/kind-types/kind-filter";
import { div } from "framer-motion/client";

export default function App() {
  const {
    clipboard_history,
    current_clipboard_item,
    selectItemToClipboard,
    deleteItem,
    activeKinds,
    toggleKindFilter,
  } = useClipboardItems();

  const {
    selectedItem,
    containerRef,
    navMessage,
    itemToSelect,
    itemToDelete,
    itemToPreview,
    clearItemToSelect,
    clearItemToDelete,
    clearItemToPreview,
    setSelectedIndex,
  } = useClipboardNavigation(clipboard_history);

  useEffect(() => {
    if (itemToSelect) {
      selectItemToClipboard(itemToSelect.content).then(() => {
        emit("overlay:close");
        clearItemToSelect();
      });
    }
  }, [itemToSelect]);

  useEffect(() => {
    if (itemToDelete) {
      deleteItem(itemToDelete.id).then(() => {
        clearItemToDelete();
      });
    }
  }, [itemToDelete])


  const isLoading = !current_clipboard_item && clipboard_history.length === 0;

  return (
    <OverlayView className="flex flex-col h-full w-full p-4 gap-3">
      <div className="w-full h-[30px] flex items-center justify-center">
        <KindFilter onToggle={toggleKindFilter} activeKinds={activeKinds} />
      </div>

      <div className="flex flex-1 overflow-hidden gap-8 px-4">
        <div className="w-fit flex flex-col gap-4">
          <span className="font-medium text-s text-gray-800 tracking-wide">
            Current Clip:
          </span>

          {isLoading && <ClipboardCardSkeleton />}

          {!isLoading && current_clipboard_item && (
            <ClipboardCard item={current_clipboard_item} />
          )}
        </div>

        <div className="w-[0.5px] bg-[#aaaaaa] h-[80%]" />

        <div className="flex-1 flex flex-col overflow-hidden">
          <div className="flex items-center justify-between px-1">
            <span className="font-medium text-s text-gray-800 tracking-wide">
              History Clip:{" "}
              <span className="font-light text-s text-gray-500 italic pr-10">
                {clipboard_history.length}/200 items
              </span>
            </span>
          </div>

          <div className="relative w-full overflow-hidden">
            <div className="w-full overflow-x-auto">
              <div
                ref={containerRef}
                className="flex flex-row gap-3 min-w-max pb-10 pt-5 px-5"
              >
                {clipboard_history.length > 0 ? clipboard_history.map((item, index) => (
                  <ClipboardCard
                    key={item.id}
                    item={item}
                    selected={item.id === selectedItem?.id}
                    handleClick={() => setSelectedIndex(index)}
                  />
                )) :

                <div>
                  <span className="font-light text-sm text-gray-500 italic">
                    No clipboard history. Start copying items or remove the filters to see them here.
                  </span>
                </div>
                }
              </div>
            </div>
            <div className="w-full flex flex-row justify-between items-center text-s text-gray-500">
              <div className="flex items-center gap-2">
                <kbd className="kbd">⏎</kbd>
                <span>Copy</span>
              </div>

              {navMessage ? (
                <div className="flex items-center gap-2 text-gray-500">
                  <kbd className="kbd">␣</kbd>
                  <span>Preview</span>
                </div>
              ) : (
                <div className="flex items-center gap-2 text-gray-500">
                  <span>Use</span>
                  <kbd className="kbd">←</kbd>
                  <kbd className="kbd">→</kbd>
                  <span>to navigate</span>
                </div>
              )}

              <div className="flex items-center gap-2">
                <kbd className="kbd">⌫</kbd>
                <span>Delete</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </OverlayView>
  );
}
