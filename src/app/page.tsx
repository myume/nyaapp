"use client";
import { invoke } from "@tauri-apps/api/core";

export default function Home() {
  return (
    <div>
      <button onClick={() => invoke("download", { id: "1996801" })}>
        hello
      </button>
    </div>
  );
}
