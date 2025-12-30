import { ClipboardItem } from "../../types/clipboardItem";
import { formatDate } from "../../utlis/date";
import { Icon } from "../../utlis/icons";

type ClipboardCardProps = {
  item: ClipboardItem;
  selected?: boolean;
  handleClick?: () => void;
};

export function ClipboardCard({ item, selected, handleClick }: ClipboardCardProps) {
  
  return (
    <div
      className={`
        relative
        min-w-[240px]
        max-w-[240px]
        rounded-2xl

        border border-black/10
        bg-white/80

        px-[20px] py-[20px]
        flex flex-col
        gap-4

        transition-colors
        ${selected ? selectedStyle : ""}
      `}

      onClick={handleClick}
    >
      <div className="flex gap-4 items-start">
        <div className="shrink-0 w-14 h-14 rounded-xl bg-white/10 flex items-center justify-center">
          <Icon type={item.kind} />
        </div>

        <p className="text-black/90 text-sm leading-snug line-clamp-3">
          {item.content}
        </p>
      </div>

      <div className="flex justify-between text-xs text-black">
        <span>{formatDate(item.created_at)}</span>
        <span>{item.content.length} characters</span>
      </div>
    </div>
  );
}


const selectedStyle =
  "ring-3 ring-blue-500";

