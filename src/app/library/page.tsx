"use client";

import { LibraryCard } from "@/components/LibraryCard";
import { LibraryDetails } from "@/components/LibraryDetails";
import { useReader } from "@/components/providers/ReaderProvider";
import { Button } from "@/components/ui/button";
import { LibraryEntry } from "@/types/LibraryEntry";
import { invoke } from "@tauri-apps/api/core";
import { ArrowLeft } from "lucide-react";
import { redirect } from "next/navigation";
import { useCallback, useEffect, useState } from "react";

export default function Library() {
  const [selectedEntry, setSelectedEntry] = useState<LibraryEntry>();
  const [fileIndex, setFileIndex] = useState<number>();
  const [library, setLibrary] = useState<LibraryEntry[]>();
  const { setReaderContext } = useReader();

  const fetchLibrary = useCallback(async () => {
    const library = await invoke<LibraryEntry[]>("list_library");
    library.sort((a, b) => a.name.localeCompare(b.name));
    setLibrary(library);
    setSelectedEntry((selectedEntry) =>
      library.find(
        (entry) =>
          entry.metafile.source.id === selectedEntry?.metafile.source.id,
      ),
    );
  }, [setLibrary, setSelectedEntry]);

  useEffect(() => {
    fetchLibrary();
  }, [fetchLibrary]);

  useEffect(() => {
    if (fileIndex === undefined || selectedEntry === undefined) return;

    setReaderContext((context) => ({
      ...context,
      fileIndex,
      libraryEntry: selectedEntry,
    }));
    redirect("/reader");
  }, [fileIndex, selectedEntry, setReaderContext]);

  return (
    <div className="flex flex-wrap gap-5">
      {selectedEntry ? (
        <div className="space-y-5">
          <div className="flex items-center gap-3">
            <Button
              variant="outline"
              onClick={() => setSelectedEntry(undefined)}
            >
              <ArrowLeft />
              Back
            </Button>
            <h1>{selectedEntry.name}</h1>
          </div>
          <LibraryDetails
            libraryEntry={selectedEntry}
            setFileIndex={setFileIndex}
            fetchLibrary={fetchLibrary}
          />
        </div>
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
