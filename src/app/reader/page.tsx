"use client";

import { useReader } from "@/components/providers/ReaderProvider";
import { Reader } from "@/components/Reader";
import Link from "next/link";

export default function ReaderPage() {
  const { readerContext } = useReader();
  let message;
  if (
    readerContext.fileIndex === undefined ||
    readerContext.libraryEntry === undefined
  ) {
    return (
      <div className="p-10 flex justify-center items-center h-screen w-full">
        <p>
          Nothing to read. Select an entry from your{" "}
          <Link href="/library" className="underline">
            library
          </Link>
        </p>
      </div>
    );
  }

  const entry = readerContext.libraryEntry.files[readerContext.fileIndex];

  if (entry === undefined) {
    message = `File ${readerContext.fileIndex} does not exist. Library entry only has ${readerContext.libraryEntry.files.length} files.`;
  }

  if (!entry.endsWith(".cbz")) {
    message = `Unable to read: ${entry} - Only CBZ files are currently supported.`;
  }

  if (message !== undefined) {
    return (
      <div className="p-10 flex justify-center items-center h-screen w-full">
        <p>{message}</p>
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
