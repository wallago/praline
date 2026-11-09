import { Counter } from "./components/Counter";

/** The page: a heading and one interactive component. */
export function App() {
  return (
    <main>
      <h1>{name}</h1>
      <Counter />
    </main>
  );
}
