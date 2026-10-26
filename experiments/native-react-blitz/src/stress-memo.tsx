import { useState, memo } from "react";
import { createRoot } from "react-dom/client";
import { Button, Card, Text, TextInput, Theme } from "@astryxdesign/core";
import { neutralTheme } from "@astryxdesign/theme-neutral";
import "./style.css";

// This control memoizes each row to isolate the cost of unrelated state updates.
const Row = memo(function Row({ index }: { index: number }) {
  return <li><Text>Task {index}: deterministic context and execution output</Text></li>;
});
// A deterministic DOM-volume benchmark, not a proxy for a full Cognate session.
const items = Array.from({ length: 200 }, (_, index) => index);
function StressProbe() {
  const [count, setCount] = useState(0);
  const [value, setValue] = useState("");
  return (
    <Theme theme={neutralTheme} mode="light">
      <main className="probe">
        <h1>Astryx — 200 memoized rows</h1>
        <Card>
          <Text>Count: {count}</Text>
          <Button label="Increment" variant="primary" onClick={() => setCount(n => n + 1)} />
          <TextInput label="Live text" value={value} onChange={setValue} />
          <Text>Echo: {value}</Text>
        </Card>
        <ul aria-label="Benchmark task rows">
          {items.map(i => <Row key={i} index={i} />)}
        </ul>
      </main>
    </Theme>
  );
}
const host = document.getElementById("app");
if (!host) throw new Error("Missing #app");
createRoot(host).render(<StressProbe />);
