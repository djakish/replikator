import { BackupEntry } from "@/lib/types";
import { json_path } from "@/lib/utils";
import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";

export default function TimeManager() {
  const [data, setData] = useState<BackupEntry[]>([]);
  const HOUR_MS = 360000;

  const fetchData = async () => {
    const data: string = await invoke("get_backups_to_update", {
      jsonPath: await json_path(),
    });
    var jsonData = JSON.parse(data);
    setData(jsonData.backups);
  };

  useEffect(() => {
    const interval = setInterval(() => {
      fetchData().catch(console.error);

      data.forEach(async (backup) => {
        invoke("increment", {
          jsonPath: await json_path(),
          hash: backup.hash,
          input: backup.input,
          output: backup.output,
          lastBackup: backup.lastBackup,
        });
      });
    }, HOUR_MS);

    return () => clearInterval(interval);
  }, [data]);

  return <></>;
}
