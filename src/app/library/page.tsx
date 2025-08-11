"use client";

import { LibraryEntry } from "@/types/LibraryEntry";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function Library() {
  const [library, setLibrary] = useState<LibraryEntry[]>();

  useEffect(() => {
    const fetchLibrary = async () => {
      const library = await invoke<LibraryEntry[]>("list_library");
      setLibrary(library);
    };
    fetchLibrary();
  }, []);
  return <div>{JSON.stringify(library)}</div>;
}
