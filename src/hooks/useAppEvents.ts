import { useEffect, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';

// Must match APP_EVENT in src-tauri/src/core/event_bridge.rs
const APP_EVENT = "app-event";

export type AppEvent = 
  | { type: "AppStarted" }
  | { type: "OverlayShown" }
  | { type: "OverlayHidden" }
  | { type: "RecordingStarted" }
  | { type: "RecordingStopped" }
  | { type: "ListeningStarted" }
  | { type: "ListeningStopped" }
  | { type: "ProcessingStarted" }
  | { type: "ProcessingFinished" }
  | { type: "AnswerReceived"; value: string }
  | { type: "HotkeyPressed"}
  | { type: "ScreenCaptured" }
  | { type: "SettingsUpdated" }
  | { type: "Error"; value: string }

export function useAppEvents(
  onEvent: (event: AppEvent) => void,
) {
  // Keep the latest callback without resubscribing on every render.
  const handler = useRef(onEvent);
  handler.current = onEvent;

  useEffect(() => {
    let cancelled = false;
    let unlisten: (() => void) | undefined;

    const setup = async () => {
      const stop = await listen<AppEvent>(
        APP_EVENT,
        (event) => {
          handler.current(event.payload);
        }
      );

      if (cancelled) {
        stop();
      } else {
        unlisten = stop;
      }
    }
    setup();

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, []);
}
