function formatDate(timestamp: number) {
  return new Date(timestamp * 1000).toLocaleString(undefined, {
    weekday: "short",
    day: "2-digit",
    month: "short",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export { formatDate };
