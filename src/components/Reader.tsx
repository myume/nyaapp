import { LibraryEntry } from "@/types/LibraryEntry";
import { invoke } from "@tauri-apps/api/core";
import Image from "next/image";
import { useEffect, useRef, useState } from "react";

export const Reader = ({
  libraryEntry,
  fileIndex,
}: {
  libraryEntry: LibraryEntry;
  fileIndex: number;
}) => {
  const [numPages, setNumPages] = useState(0);
  const [currentPage, setCurrentPage] = useState(0);
  const pagesRef = useRef<(HTMLImageElement | null)[]>([]);
  const observer = useRef<IntersectionObserver | null>(null);
  const [lastReadPage] = useState(5);

  useEffect(() => {
    (async () => {
      const numPages = await invoke<number>("load_cbz", {
        id: libraryEntry.metafile.source.id,
        fileNum: fileIndex,
      });
      setNumPages(numPages);
    })();
  }, [fileIndex, libraryEntry]);

  useEffect(() => {
    pagesRef.current[lastReadPage]?.scrollIntoView({ behavior: "instant" });
  }, [numPages, lastReadPage]);

  useEffect(() => {
    observer.current = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            const page = entry.target.getAttribute("data-page");
            if (page) {
              setCurrentPage(parseInt(page));
            }
          }
        }
      },
      { threshold: 0.4 },
    );

    for (const item of pagesRef.current) {
      if (item) {
        observer.current.observe(item);
      }
    }

    return () => {
      if (observer.current) {
        observer.current.disconnect();
      }
    };
  }, [numPages]);

  return (
    <div>
      <div className="grid">
        {Array.from({ length: numPages }, (_, i) => i).map((i) => (
          <Image
            key={i}
            ref={(el) => {
              pagesRef.current[i] = el;
            }}
            data-page={i}
            src={`pages://localhost/${libraryEntry.metafile.source.id}/${fileIndex}/${i}`}
            alt={`Page ${i + 1}`}
            className="w-full"
            style={{ objectFit: "contain" }}
            height={500}
            width={500}
            quality={100}
          />
        ))}
      </div>
      {numPages > 0 && (
        <div className="fixed bottom-2 right-2 text-muted-foreground text-[0.7rem]">
          {currentPage + 1} / {numPages}
        </div>
      )}{" "}
    </div>
  );
};
