"use client";

import { Metadata } from "@/types/Metadata";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function Library() {
  const [library, setLibrary] = useState<Metadata[]>();

  useEffect(() => {
    const fetchLibrary = async () => {
      const library = await invoke<Metadata[]>("list_library");
      setLibrary(library);
    };
    fetchLibrary();
  }, []);
  return <div>{JSON.stringify(library)}</div>;
}
