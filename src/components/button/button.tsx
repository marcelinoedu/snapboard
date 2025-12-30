import "./button.css"
interface ButtonProps {
  onClick?: () => void;
  disabled?: boolean;
  children: React.ReactNode;
  className?: string;
}

export default function Button({
  onClick,
  disabled = false,
  children,
  className = "",
}: ButtonProps) {
  return (
    <button
      className={`apple-button ${className}`}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
}
