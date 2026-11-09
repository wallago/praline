import { useState } from "react";

/** A button that counts its own clicks. */
export function Counter() {
  const [count, setCount] = useState(0);
  return (
    <button type="button" onClick={() => setCount((n) => n + 1)}>
      Clicked {count} {count === 1 ? "time" : "times"}
    </button>
  );
}
