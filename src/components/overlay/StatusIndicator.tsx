import type { AssistantStatus } from "./Overlay";

interface StatusIndicatorProps {
  status: AssistantStatus;
}

const statusConfig = {
  idle: {
    label: "Ready",
    dot: "bg-zinc-400",
  },

  listening: {
    label: "Listening",
    dot: "bg-green-400 animate-pulse",
  },

  processing: {
    label: "Thinking",
    dot: "bg-yellow-400 animate-pulse",
  },

  answer: {
    label: "Answer",
    dot: "bg-blue-400",
  },

  error: {
    label: "Error",
    dot: "bg-red-400",
  },
};

export function StatusIndicator({ status }: StatusIndicatorProps) {
  const config = statusConfig[status];

  return (
    <div className="flex items-center gap-2">
      <span className={`h-2 w-2.5 rounded-full ${config.dot}`} />
      <span className="text-sm font-medium text-white">{config.label}</span>
    </div>
  );
}
