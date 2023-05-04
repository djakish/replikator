import { appDataDir } from "@tauri-apps/api/path";

export async function json_path() {
  let result = await appDataDir();
  return result + "backup_entries.json";
}

export interface Payload {
  message: string;
}
