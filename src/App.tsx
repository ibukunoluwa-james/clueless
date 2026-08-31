
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Overlay } from "./components/overlay/Overlay";
import Dashboard from "./components/dashboard/Dashboard";


function App() {
  const currentWindow = getCurrentWindow();

  if (currentWindow.label === "overlay") {
    return <Overlay />
  }

  return <Dashboard />
}

export default App;
