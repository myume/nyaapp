"use client";

import { useReader } from "@/components/providers/ReaderProvider";
import { Reader } from "@/components/Reader";

export default function ReaderPage() {
  const { readerContext } = useReader();
  if (
    readerContext.fileIndex === undefined ||
    readerContext.libraryEntry === undefined
  ) {
    return (
      <div className="p-2">
        Nothing to read. Select an entry from your library
      </div>
    );
  }

  return (
    <Reader
      fileIndex={readerContext.fileIndex}
      libraryEntry={readerContext.libraryEntry}
    />
  );
}
