import { useState } from "react";

import { StatusIndicator } from "./StatusIndicator";
import { AnswerCard } from "./AnswerCard";
import { useAppEvents, type AppEvent } from "../../hooks/useAppEvents";

export type AssistantStatus =
  "idle" | "listening" | "processing" | "answer" | "error";



export function Overlay() {
  const [status, setStatus] = useState<AssistantStatus>("idle");

  const [answer, setAnswer] = useState<string>();

  const handleEvent = (event: AppEvent) => {
    switch (event.type) {
      case "ListeningStarted":
        setStatus("listening");
        console.log("HI")
        break;

      case "ProcessingStarted":
        setStatus("processing");
        break;

      case "AnswerReceived":
        setStatus("answer");
        setAnswer(event.value);
        break;

      case "Error":
        setStatus("error");
        setAnswer(event.value);
        break;

      case "ListeningStopped":
        setStatus("idle");
        break;
    }
  };

  useAppEvents(handleEvent);

  return (
    <div className="pointer-events-none flex min-h-screen w-full items-start justify-end p-6">
      <div className="pointer-events-auto w-[360px] overflow-hidden rounded-2xl border border-white/10 bg-black/80 shadow-2xl backdrop-blur-xl">
        <div className="p-4">
          <StatusIndicator status={status} />

          {answer && (
            <div className="mt-4">
              <AnswerCard answer={answer} />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
