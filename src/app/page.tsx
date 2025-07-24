"use client";
import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";

export default function Home() {
  return (
    <div>
      <button
        onClick={async () => {
          const results = await invoke("search", { query: "c=3_1" });
        }}
      >
        hello
      </button>
    </div>
  );
}
