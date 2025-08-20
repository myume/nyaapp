"use client";

import { LibraryCard } from "@/components/LibraryCard";
import { LibraryDetails } from "@/components/LibraryDetails";
import { useReader } from "@/components/providers/ReaderProvider";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { LibraryEntry } from "@/types/LibraryEntry";
import { invoke } from "@tauri-apps/api/core";
import { ArrowLeft, RotateCcw, Settings } from "lucide-react";
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
          <div className="flex justify-between items-center">
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
            <DropdownMenu>
              <DropdownMenuTrigger className="hover:bg-background hover:cursor-pointer p-1 rounded-full aspect-square transition-colors duration-200">
                <Settings size={20} />
              </DropdownMenuTrigger>
              <DropdownMenuContent className="mx-2">
                <DropdownMenuItem
                  onClick={async () => {
                    await invoke("clear_reading_progress", {
                      id: selectedEntry.metafile.source.id,
                    });
                    fetchLibrary();
                  }}
                >
                  <RotateCcw className="text-red-400" />
                  <h1 className="text-red-400 hover:text-red-400">
                    Reset ALL Reading Progress
                  </h1>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
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
