import { StatusIndicator } from "./StatusIndicator";
import { AnswerCard } from "./AnswerCard";

export type AssistantStatus =
  | "idle"
  | "listening"
  | "processing"
  | "answer"
  | "error";

interface OverlayProps {
  status?: AssistantStatus;
  answer?: string;
}

export function Overlay({
  status = "idle",
  answer,
}: OverlayProps) {

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
  )
}
