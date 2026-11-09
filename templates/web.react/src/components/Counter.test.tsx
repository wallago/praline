import { fireEvent, render, screen } from "@testing-library/react";
import { expect, test } from "vitest";
import { Counter } from "./Counter";

test("starts at zero", () => {
  render(<Counter />);
  expect(screen.getByRole("button").textContent).toBe("Clicked 0 times");
});

test("counts each click", () => {
  render(<Counter />);
  const button = screen.getByRole("button");
  fireEvent.click(button);
  expect(button.textContent).toBe("Clicked 1 time");
  fireEvent.click(button);
  expect(button.textContent).toBe("Clicked 2 times");
});
