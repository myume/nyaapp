import { LibraryEntry } from "@/types/LibraryEntry";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Image from "next/image";
import { useEffect, useState } from "react";

export const Reader = ({
  libraryEntry,
  fileIndex,
}: {
  libraryEntry: LibraryEntry;
  fileIndex: number;
}) => {
  const [pages, setPages] = useState<string[]>([]);

  useEffect(() => {
    let unlisten: () => void;

    const setupListener = async () => {
      unlisten = await listen<string>("page-read", ({ payload: page }) => {
        setPages((pages) => {
          if (pages.includes(page)) {
            return pages;
          }
          return [...pages, page];
        });
      });

      await invoke("read_cbz", {
        path: `${libraryEntry.output_dir}/${libraryEntry.files[fileIndex]}`,
      });
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [fileIndex, libraryEntry]);

  return (
    <div className="grid">
      {pages.map((page, i) => (
        <Image
          src={`data:image/*;base64,${page}`}
          alt={`Page ${i + 1}`}
          className="w-full"
          key={i}
          height={100}
          width={100}
          quality={100}
        />
      ))}
    </div>
  );
};
