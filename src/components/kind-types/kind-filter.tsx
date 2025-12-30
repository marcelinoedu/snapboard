import { useKindTypes } from "../../hooks/useKindTypes";

type KindFilterProps = {
  activeKinds: string[];
  onToggle: (kind: string) => void;
};

export function KindFilter({
  activeKinds,
  onToggle,
}: KindFilterProps) {
  const { kindTypes: kinds } = useKindTypes();

  return (
    <div className="w-full h-full flex items-center justify-center gap-4">
      {kinds.map((kind) => {
        const selected = activeKinds.includes(kind.value);

        return (
          <button
            key={kind.value}
            onClick={() => onToggle(kind.value)}
            className={`
              flex items-center gap-2
              px-3 py-1
              rounded-full
              text-s font-medium
              transition-all

              ${
                selected
                  ? "bg-gray-200 text-gray-900 ring-1 ring-gray-300"
                  : "text-gray-600 hover:bg-gray-100"
              }
            `}
          >
            <span
              className={`
                w-3 h-3 rounded-full ${kind.color}
              `}
            />
            <span>{kind.label}</span>
          </button>
        );
      })}
    </div>
  );
}
