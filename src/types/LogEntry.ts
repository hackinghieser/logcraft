export default interface LogEntry {
  timestamp: string;
  level: string;
  message: string;
  template?: string;
  exception?: string;
  eventId?: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  properties?: Record<string, any>;
}
