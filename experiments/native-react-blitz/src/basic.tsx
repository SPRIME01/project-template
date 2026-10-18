import { useState } from "react";
import { createRoot } from "react-dom/client";

function ReactCompatibilityProbe() {
  const [count, setCount] = useState(0);
  const [value, setValue] = useState("");
  return (
    <main>
      <h1>React DOM / Boa / Blitz</h1>
      <p>Count: {count}</p>
      <button type="button" onClick={() => setCount((prev) => prev + 1)}>Increment</button>
      <label htmlFor="typing">Text</label>
      <input id="typing" value={value} onChange={(event) => setValue(event.target.value)} />
      <p>Echo: {value}</p>
    </main>
  );
}
const root = document.getElementById("app");
if (!root) throw new Error("Missing root");
createRoot(root).render(<ReactCompatibilityProbe />);
