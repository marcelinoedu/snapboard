import { IconType } from "../types/iconTypes";
import { useKindTypes } from "../hooks/useKindTypes";
import { JSX } from "react";


type IconProps = {
  type: IconType;
  size?: number;
  className?: string;
};

export function Icon({
  type,
  size = 20,
  className = "",
}: IconProps): JSX.Element {
  const containerSize = size + 16;

  const { kindTypes } = useKindTypes();

  return (
    <div
      className={`
        relative
        flex items-center justify-center
        rounded-[22%]
        ${kindTypes.find(kind => kind.value === type)?.color}
        ${className}
      `}
      style={{
        width: containerSize,
        height: containerSize,
      }}
    >
      {/* sombra interna (relevo Apple) */}
      <div
        className="
          absolute inset-0
          rounded-[22%]

        "
      />

      {/* highlight superior esquerdo */}
      <div
        className="
          absolute inset-0
          rounded-[22%]
          bg-gradient-to-br
          from-white/55
          via-white/12
          to-transparent
          pointer-events-none
        "
      />

      {/* glyph */}
      <svg
        width={size}
        height={size}
        viewBox="0 0 24 24"
        fill="none"
        stroke="white"
        strokeWidth={1.6}
        strokeLinecap="round"
        strokeLinejoin="round"
        className="relative z-10 drop-shadow-[0_1px_0_rgba(0,0,0,0)]"
      >
        {renderPath(type)}
      </svg>
    </div>
  );
}

function renderPath(type: IconType): JSX.Element {
  switch (type) {
    case "text":
      return (
        <>
          <path d="M4 6h16" />
          <path d="M4 12h10" />
          <path d="M4 18h8" />
        </>
      );

    case "image":
      return (
        <>
          <rect x="3" y="4" width="18" height="16" rx="2" />
          <circle cx="8.5" cy="9.5" r="1.5" />
          <path d="M21 15l-5-5L5 21" />
        </>
      );

    case "file":
      return (
        <>
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
          <path d="M14 2v6h6" />
        </>
      );

    case "code":
      return (
        <>
          <path d="M8 9l-4 3 4 3" />
          <path d="M16 9l4 3-4 3" />
        </>
      );

    case "link":
      return (
        <>
          <path d="M10 13a5 5 0 0 0 7.07 0l1.83-1.83a5 5 0 0 0-7.07-7.07L10 5" />
          <path d="M14 11a5 5 0 0 0-7.07 0L5.1 12.83a5 5 0 0 0 7.07 7.07L14 19" />
        </>
      );


    case "location":      
    return (
        <>
          <path d="M12 21s8-4.434 8-10a8 8 0 1 0-16 0c0 5.566 8 10 8 10z" />
          <circle cx="12" cy="11" r="3" />
        </>
      );
  }
}
