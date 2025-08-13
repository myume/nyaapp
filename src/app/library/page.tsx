"use client";

import { LibraryCard } from "@/components/LibraryCard";
import { LibraryDetails } from "@/components/LibraryDetails";
import { Reader } from "@/components/Reader";
import { Button } from "@/components/ui/button";
import { LibraryEntry } from "@/types/LibraryEntry";
import { invoke } from "@tauri-apps/api/core";
import { ArrowLeft } from "lucide-react";
import { useEffect, useState } from "react";

export default function Library() {
  const [selectedEntry, setSelectedEntry] = useState<LibraryEntry | null>(null);
  const [fileIndex, setFileIndex] = useState<number | null>(null);
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
      {selectedEntry ? (
        fileIndex !== null ? (
          <div className="w-full">
            <div className="flex items-center gap-3">
              <Button variant="outline" onClick={() => setFileIndex(null)}>
                <ArrowLeft />
                Back
              </Button>
            </div>

            <Reader fileIndex={fileIndex} libraryEntry={selectedEntry} />
          </div>
        ) : (
          <div className="space-y-5">
            <div className="flex items-center gap-3">
              <Button variant="outline" onClick={() => setSelectedEntry(null)}>
                <ArrowLeft />
                Back
              </Button>
              <h1>{selectedEntry.name}</h1>
            </div>
            <LibraryDetails
              libraryEntry={selectedEntry}
              setFileIndex={setFileIndex}
            />
          </div>
        )
      ) : (
        library?.map((entry) => (
          <LibraryCard
            key={entry.metafile.source.id}
            libraryEntry={entry}
            onDeleteAction={(id) => {
              setLibrary((library) =>
                library?.filter(({ metafile: { source } }) => source.id !== id),
              );
            }}
            setSelectedAction={setSelectedEntry}
          />
        ))
      )}
    </div>
  );
}
