import { invoke } from "@tauri-apps/api/core";

function App() {
  const showOverlay = async () => {
    console.log("Calling show_overlay...");
  
    try {
      const result = await invoke("show_overlay");
  
      console.log("Rust returned:", result);
    } catch (error) {
      console.error("Tauri command failed:", error);
    }
  };

  const hideOverlay = async () => {
    try {
      await invoke("hide_overlay");
      console.log("Overlay hidden");
    } catch (error) {
      console.error("Failed to hide overlay:", error);
    }
  };

  const toggleOverlay = async () => {
    try {
      await invoke("toggle_overlay");
      console.log("Overlay toggled");
    } catch (error) {
      console.error("Failed to toggle overlay:", error);
    }
  };

  return (
    <div>
      <h1>Clueless</h1>

      <button onClick={showOverlay}>
        Show Overlay
      </button>

      <button onClick={hideOverlay}>
        Hide Overlay
      </button>

      <button onClick={toggleOverlay}>
        Toggle Overlay
      </button>
    </div>
  );
}

export default App;