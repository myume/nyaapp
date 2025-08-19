"use client";

import { invoke } from "@tauri-apps/api/core";
import Image from "next/image";
import { useEffect, useRef, useState } from "react";
import { Virtuoso, VirtuosoHandle } from "react-virtuoso";
import { useReader } from "../providers/ReaderProvider";
import { ReaderToolbar } from "./ReaderToolbar";

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
  const readingProgressTimeout = useRef<NodeJS.Timeout | null>(null);
  const [dimensions, setDimensions] = useState<[number, number][]>([]);
  const virtuoso = useRef<VirtuosoHandle | null>(null);
  const [windowWidth, setWindowWidth] = useState(0);
  const observer = useRef<IntersectionObserver | null>(null);

  useEffect(() => {
    observer.current = new IntersectionObserver(
      (entries: IntersectionObserverEntry[]) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            const page = Number.parseInt(
              entry.target.getAttribute("data-page") ?? "0",
            );
            setCurrentPage(page);
          }
        }
      },
      {
        threshold: 0.5,
      },
    );

    return () => {
      observer.current?.disconnect();
    };
  }, []);

  useEffect(() => {
    const handleResize = () => {
      setWindowWidth(window.innerWidth);
      virtuoso.current?.scrollToIndex(currentPage);
    };
    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, [currentPage]);

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

  return (
    <div className="relative">
      <div className="absolute top-0 w-full z-10 opacity-0 hover:opacity-100 transition-opacity duration-300">
        <ReaderToolbar
          currentPage={currentPage}
          numPages={numPages}
          setCurrentPageAction={(page) => {
            setCurrentPage(page);
            virtuoso.current?.scrollToIndex({
              index: page,
              behavior: "smooth",
            });
          }}
        />
      </div>
      <Virtuoso
        key={windowWidth}
        ref={virtuoso}
        style={{ height: "100vh" }}
        totalCount={numPages}
        initialTopMostItemIndex={currentPage}
        increaseViewportBy={2000}
        itemContent={(i) => (
          <div
            data-page={i}
            ref={(el) => {
              if (el) {
                observer.current?.observe(el);
              }
            }}
          >
            <Image
              key={i}
              src={`pages://localhost/${libraryEntry.metafile.source.id}/${fileIndex}/${i}`}
              alt={`Page ${i + 1}`}
              className="m-auto w-full xl:w-1/2"
              style={{ objectFit: "contain" }}
              height={dimensions[i]?.[1] || 1000}
              width={dimensions[i]?.[0] || 500}
              quality={100}
              priority
            />
          </div>
        )}
      />
      {numPages > 0 && (
        <div className="absolute bottom-2 right-2 text-muted-foreground text-[0.7rem]">
          {currentPage + 1} / {numPages}
        </div>
      )}
    </div>
  );
};
