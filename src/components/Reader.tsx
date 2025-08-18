"use client";

import { info } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";
import Image from "next/image";
import { useEffect, useRef, useState } from "react";
import { useReader } from "./providers/ReaderProvider";

export const Reader = () => {
  const { readerContext, setReaderContext } = useReader();
  let { libraryEntry, fileIndex } = readerContext;
  libraryEntry = libraryEntry!;
  fileIndex = fileIndex!;

  const filename = libraryEntry.files[fileIndex];

  const [numPages, setNumPages] = useState(0);
  const [currentPage, setCurrentPage] = useState(
    libraryEntry.metafile.reading_progress[filename]?.current_page ?? 0,
  );
  const pagesRef = useRef<(HTMLImageElement | null)[]>([]);
  const observer = useRef<IntersectionObserver | null>(null);
  const readingProgressTimeout = useRef<NodeJS.Timeout | null>(null);
  const [dimensions, setDimensions] = useState<[number, number][]>([]);

  useEffect(() => {
    (async () => {
      const numPages = await invoke<number>("load_cbz", {
        id: libraryEntry.metafile.source.id,
        fileNum: fileIndex,
      });
      setNumPages(numPages);
      const dimensions = await invoke<[number, number][]>("get_dimensions", {
        id: libraryEntry.metafile.source.id,
        fileNum: fileIndex,
      });
      setDimensions(dimensions);
    })();
  }, [fileIndex, libraryEntry]);

  useEffect(() => {
    const lastReadPage =
      libraryEntry.metafile.reading_progress[filename]?.current_page ?? 0;
    info("Restoring reading progress");
    pagesRef.current[lastReadPage]?.scrollIntoView({ behavior: "instant" });
  }, [numPages, filename, libraryEntry.metafile.reading_progress]);

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

  useEffect(() => {
    if (readingProgressTimeout.current)
      clearTimeout(readingProgressTimeout.current);

    readingProgressTimeout.current = setTimeout(async () => {
      await invoke("update_reading_progress", {
        id: libraryEntry.metafile.source.id,
        fileNum: fileIndex,
        updatedPage: currentPage,
      });
    }, 500);

    return () => {
      setReaderContext((context) => {
        const updatedContext = { ...context };
        updatedContext.libraryEntry!.metafile.reading_progress[filename] = {
          current_page: currentPage,
          total_pages: numPages,
        };
        return updatedContext;
      });
    };
  }, [
    currentPage,
    fileIndex,
    libraryEntry.metafile.source.id,
    filename,
    numPages,
    setReaderContext,
  ]);

  useEffect(() => {
    const handleResize = () => {
      pagesRef.current[currentPage]?.scrollIntoView({ behavior: "instant" });
    };

    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, [currentPage]);

  return (
    <>
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
            className="m-auto w-full xl:w-1/2"
            style={{ objectFit: "contain" }}
            height={dimensions[i]?.[1] ?? 1000}
            width={dimensions[i]?.[0] ?? 500}
            quality={100}
          />
        ))}
      </div>
      {numPages > 0 && (
        <div className="fixed bottom-2 right-2 text-muted-foreground text-[0.7rem]">
          {currentPage + 1} / {numPages}
        </div>
      )}
    </>
  );
};
