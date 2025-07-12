export default interface LogEntry {
  "@t"?: string;
  "@m"?: string;
  "@mt": string;
  "@l"?: string;
  "@x"?: string;
  "@i"?: string;
  "@r": string[];
  "properties": Record<string, any>
}
