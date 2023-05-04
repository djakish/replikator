export type BackupEntry = {
  title: string;
  input: string;
  output: string;
  lastBackup: Date;
  nextUpdate: number;
  deleteButton: "";
  backupButton: "";
  hash: string;
};

export type BackupReturn = {
  backups: BackupEntry[];
};

export enum UpdateTime {
  Never = 9999,
  Week = 7,
  Month = 30,
}
