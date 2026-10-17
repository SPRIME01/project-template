import { useState } from "react";
import { createRoot } from "react-dom/client";
import { Button, Card, Text, TextInput, Theme } from "@astryxdesign/core";
import { neutralTheme } from "@astryxdesign/theme-neutral";
import "./style.css";

function App() {
  const [count, setCount] = useState(0);
  const [value, setValue] = useState("");
  return (
    <Theme theme={neutralTheme} mode="light">
      <main className="probe">
        <h1>React 19 / Astryx / Boa / Blitz</h1>
        <Card>
          <Text>Count: {count}</Text>
          <Button label="Increment" variant="primary" onClick={() => setCount((v) => v + 1)} />
          <TextInput label="Live text" value={value} onChange={setValue} />
          <Text>Echo: {value}</Text>
        </Card>
      </main>
    </Theme>
  );
}
const root = document.getElementById("app");
if (!root) throw new Error("Missing #app");
createRoot(root).render(<App />);
