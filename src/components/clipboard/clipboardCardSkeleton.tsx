export function ClipboardCardSkeleton() {
  return (
    <div
      className="
        w-full
        rounded-2xl
        px-4 py-3
        flex flex-col
        gap-3
        bg-white/10
        animate-pulse
      "
    >
      <div className="flex gap-4">
        <div className="w-12 h-12 rounded-xl bg-white/20" />

        <div className="flex-1 space-y-2">
          <div className="h-3 bg-white/20 rounded w-full" />
          <div className="h-3 bg-white/20 rounded w-5/6" />
          <div className="h-3 bg-white/20 rounded w-2/3" />
        </div>
      </div>

      <div className="flex justify-between mt-1">
        <div className="h-3 bg-white/20 rounded w-24" />
        <div className="h-3 bg-white/20 rounded w-16" />
      </div>
    </div>
  );
}
