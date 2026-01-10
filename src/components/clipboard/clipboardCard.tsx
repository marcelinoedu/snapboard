import { ClipboardItem } from "../../types/clipboardItem";
import { formatDate } from "../../utlis/date";
import { Icon } from "../../utlis/icons";

type ClipboardCardProps = {
  item: ClipboardItem;
  selected?: boolean;
  handleClick?: () => void;
};

export function ClipboardCard({
  item,
  selected = false,
  handleClick,
}: ClipboardCardProps) {
  return (
    <div
      onClick={handleClick}
      className={`
        relative
        min-w-[240px]
        max-w-[240px]
        min-h-[200px]
        rounded-2xl
        bg-[#3c3d43]
        px-[13px] py-[13px]
        flex flex-col
        cursor-pointer
        transition-all
        duration-200
        hover:bg-[#2E2F38]
        ${selected ? selectedStyle : ""}
      `}
    >
      {/* ───────────────── Header ───────────────── */}
      <div className="h-[40px] flex items-start justify-between">
        <div className="flex items-start gap-3 min-w-0">
          <div className="shrink-0 rounded-xl flex items-center justify-center">
            <Icon type={item.kind} />
          </div>

          <span className="text-white text-sm truncate capitalize pt-1">
            {item.kind}
          </span>
        </div>

        <span className="text-white/60 text-xs pt-1 whitespace-nowrap">
          {formatDate(item.created_at)}
        </span>
      </div>

      
      <div className="flex-1 rounded-xl p-3 overflow-hidden text-sm">
        {item.content.length > 100
          ? item.content.slice(0, 100) + "..."
          : item.content}
      </div>

      <div className="h-[36px] flex items-center justify-center text-sm text-white/70 rounded-b-xl px-3">
       { selected && 
       <div className="flex items-center gap-2">
        <kbd className="kbd">␣</kbd>
        <span>Preview</span>
        </div>
       }
      </div>
    </div>
  );
}

const selectedStyle = `
  ring-2
  ring-white-500/60
  bg-[#2E2F38]
`;
