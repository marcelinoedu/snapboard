import { z } from "zod";

export const ClipboardItemSchema = z.object({
  id: z.string(),
  kind: z.enum(["text", "image", "code", "file", "link", "location"]),
  content: z.string(),
  filepath: z.string().optional(),
  created_at: z.number(), 
  expires_at: z.number().optional(),
});


export type ClipboardItem = z.infer<typeof ClipboardItemSchema>;
export const ClipboardItemArraySchema = z.array(ClipboardItemSchema);