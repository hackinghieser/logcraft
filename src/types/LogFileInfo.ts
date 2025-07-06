export default interface LogFileInfo {
  path: string;
  totalCount: number;
  logLevels: string[];
  dateRange?: [string, string];
}
