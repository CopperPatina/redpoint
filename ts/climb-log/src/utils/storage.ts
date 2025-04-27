export function safeJSONParse<T>(key: string, fallback: T): T {
    const value = localStorage.getItem(key);
    try {
      if (!value) return fallback;
      return JSON.parse(value);
    } catch (e) {
      console.error(`Failed to parse localStorage key "${key}":`, e);
      localStorage.removeItem(key);
      return fallback;
    }
  }