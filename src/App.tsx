import { useState } from "react";
// import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import TitleBar from "./components/common/TitleBar.tsx";

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  return (
      <>
        <TitleBar/>
      </>
  );
}

export default App;
