import { LibraryEntry } from "@/types/LibraryEntry";
import { invoke } from "@tauri-apps/api/core";
import Image from "next/image";
import { useEffect, useState } from "react";

export const Reader = ({
  libraryEntry,
  fileIndex,
}: {
  libraryEntry: LibraryEntry;
  fileIndex: number;
}) => {
  const [numPages, setNumPages] = useState(0);

  useEffect(() => {
    (async () => {
      const numPages = await invoke<number>("load_cbz", {
        id: libraryEntry.metafile.source.id,
        fileNum: fileIndex,
      });
      setNumPages(numPages);
    })();
  }, [fileIndex, libraryEntry]);

  return (
    <div className="grid">
      {Array.from({ length: numPages }, (_, i) => i).map((i) => (
        <Image
          key={i}
          src={`pages://localhost/${libraryEntry.metafile.source.id}/${fileIndex}/${i}`}
          alt={`Page ${i + 1}`}
          className="w-full"
          height={1500}
          width={500}
          quality={100}
        />
      ))}
    </div>
  );
};
