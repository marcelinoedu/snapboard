export type KindType = {
  value: string;
  label: string;
  color: string;
};

export function useKindTypes() {
  const kindTypes: KindType[] = [
    { label: "Text", value: "text", color: "bg-blue-600" },
    { label: "Image", value: "image", color: "bg-pink-500" },
    { label: "File", value: "file", color: "bg-orange-600" },
    { label: "code", value: "code", color: "bg-[#0fa585]" },
    { label: "Link", value: "link", color: "bg-rose-600" },
    { label: "Location", value: "location", color: "bg-green-600" },
  ];

  return { kindTypes};
}





