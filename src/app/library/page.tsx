"use client";

import { LibraryCard } from "@/components/LibraryCard";
import { LibraryEntry } from "@/types/LibraryEntry";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function Library() {
  const [library, setLibrary] = useState<LibraryEntry[]>();

  useEffect(() => {
    const fetchLibrary = async () => {
      const library = await invoke<LibraryEntry[]>("list_library");
      library.sort((a, b) => a.name.localeCompare(b.name));
      setLibrary(library);
    };
    fetchLibrary();
  }, []);

  return (
    <div className="flex flex-wrap gap-5">
      {library?.map((entry) => (
        <LibraryCard
          key={entry.metafile.source.id}
          libraryEntry={entry}
          onDeleteAction={(id) => {
            setLibrary((library) =>
              library?.filter(({ metafile: { source } }) => source.id !== id),
            );
          }}
        />
      ))}
    </div>
  );
}
