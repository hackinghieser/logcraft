import { Filters } from "../types";

interface LogEntry {
  timestamp: string;
  level: string;
  message: string;
  template?: string;
  exception?: string;
  eventId?: string;
  properties?: Record<string, any>;
}

self.onmessage = (event: MessageEvent) => {
  const { logEntries, filters } = event.data;
  let filtered = [...logEntries];

  // Filter by log levels
  if (filters.selectedLevels.length > 0) {
    filtered = filtered.filter((entry: LogEntry) =>
      filters.selectedLevels.includes(entry.level),
    );
  }

  // Filter by search text
  if (filters.searchText.trim()) {
    const searchTerm = filters.searchText.toLowerCase();
    filtered = filtered.filter(
      (entry: LogEntry) =>
        entry.message.toLowerCase().includes(searchTerm) ||
        entry.template?.toLowerCase().includes(searchTerm) ||
        entry.level.toLowerCase().includes(searchTerm),
    );
  }

  // Filter by date range
  if (filters.dateRange.length === 2) {
    const [startDate, endDate] = filters.dateRange;
    filtered = filtered.filter((entry: LogEntry) => {
      const entryDate = new Date(entry.timestamp);
      return entryDate >= startDate && entryDate <= endDate;
    });
  }

  self.postMessage(filtered);
};
