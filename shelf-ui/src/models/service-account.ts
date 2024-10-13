export interface ServiceAccount {
  id: number;
  name: string;
  expires_at: Date | undefined;
  created_at: Date;
  updated_at: Date;
  repo_count: number;
}
