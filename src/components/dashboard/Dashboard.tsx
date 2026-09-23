
import { type AssistantStatus } from "./../overlay/Overlay";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
function Dashboard() {
  
  
  const [overlayVisible, setOverlayVisible] = useState(false);

  const [status, setStatus] = useState<AssistantStatus>("idle");

  const [answer, setAnswer] = useState<string>();

  const showOverlay = async () => {
    try {
      await invoke("show_overlay");
      setOverlayVisible(true);
    } catch (error) {
      console.error("Failed to hide overlay:", error);
    }
  };

  const hideOverlay = async () => {
      try {
        await invoke("hide_overlay");
        setOverlayVisible(false);
      } catch (error) {
        console.error("Failed to hide overlay:", error);
      }
    };

  const testListening = async () => {
    try {
      await invoke("test_listening");
    } catch (error) {
      console.error(error)
    }
  };

  const testProcessing = () => {
    setStatus("processing");
  };

  const testAnswer = () => {
    setStatus("answer");

    setAnswer(
      "Tcp is connection-oriented, meaning it establishes a connection before transmitting data and guarantees ordered delivery. UDP is connectionless and prioritizes speed over guaranteed delivery.",
    );
  };

  return (
    <div className="min-h-screen bg-zinc-950 p-8 text-white">
      <div className="mx-auto max-w-3xl">
        <h1 className="text-3xl font-bold">Clueless</h1>
        <p className="mt-2 text-zinc-400">AI meeting assistant</p>
        <div className="mt-8 flex flex-wrap gap-3">
          <button
            onClick={showOverlay}
            className="rounded-lg bg-white px-4 py-2 text-sm font-medium text-black transition hover:bg-zinc-200"
          >
            Show Overlay
          </button>
          <button
            onClick={hideOverlay}
            className="rounded-lg border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium transition hover:bg-white/10"
          >
            Hide Overlay
          </button>

          <button
            onClick={testListening}
            className="rounded-lg border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium transition hover:bg-white/10"
          >
            Test Listening
          </button>
          <button
            onClick={testProcessing}
            className="rounded-lg border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium transition hover:bg-white/10"
          >
            Test Processing
          </button>

          <button
            onClick={testAnswer}
            className="rounded-lg border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium transition hover:bg-white/10"
          >
            Test Answer
          </button>
        </div>
        <div className="mt-8 rounded-xl border border-white/10 bg-white/5 p-4">
          <p className="text-sm text-zinc-400">Overlay state</p>

          <p className="mt-1 font-mono text-sm">
            {overlayVisible ? "visible" : "hidden"}
          </p>
        </div>
      </div>
    </div>
  );

}

export default Dashboard;