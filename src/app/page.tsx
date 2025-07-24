"use client";
import { invoke } from "@tauri-apps/api/core";

export default function Home() {
  return (
    <div>
      <button onClick={() => invoke("search", { query: "c=3_1" })}>
        hello
      </button>
    </div>
  );
}
