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
  const [pages, setPages] = useState<string[]>([]);

  useEffect(() => {
    (async () => {
      const pages = await invoke<string[]>("read_cbz", {
        path: `${libraryEntry.output_dir}/${libraryEntry.files[fileIndex]}`,
      });
      setPages(pages);
    })();
  }, [fileIndex]);

  return (
    <div>
      {pages.map((page, i) => (
        <Image
          src={`data:image/*;base64,${page}`}
          alt={"Page " + i}
          className="w-full"
          key={i}
          height={100}
          width={100}
          quality={100}
          priority
        />
      ))}
    </div>
  );
};
