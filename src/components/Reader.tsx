"use client";

import { info } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";
import Image from "next/image";
import { useEffect, useRef, useState, useCallback } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";
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
  const [dimensions, setDimensions] = useState<[number, number][]>([]);
  const readingProgressTimeout = useRef<NodeJS.Timeout | null>(null);
  const parentRef = useRef<HTMLDivElement>(null);

  const virtualizer = useVirtualizer({
    count: numPages,
    getScrollElement: () => parentRef.current,
    estimateSize: useCallback(
      (index: number) => {
        // Use actual dimensions if available, otherwise estimate
        if (dimensions[index]) {
          const [width, height] = dimensions[index];
          const containerWidth = parentRef.current?.clientWidth ?? 1000;
          // Calculate the actual display width based on responsive classes
          const isXLScreen = containerWidth >= 1280; // xl breakpoint
          const displayWidth = isXLScreen
            ? containerWidth * 0.5
            : containerWidth;
          const scaleFactor = displayWidth / width;
          return Math.ceil(height * scaleFactor);
        }
        return 1000;
      },
      [dimensions],
    ),
    overscan: 3,
  });

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
    if (numPages > 0) {
      const lastReadPage =
        libraryEntry.metafile.reading_progress[filename]?.current_page ?? 0;
      info(`Restoring reading progress to page ${lastReadPage}`);
      virtualizer.scrollToIndex(lastReadPage, { align: "start" });
    }
  }, [numPages, filename, libraryEntry.metafile.reading_progress, virtualizer]);

  useEffect(() => {
    if (!parentRef.current) return;

    const handleScroll = () => {
      const items = virtualizer.getVirtualItems();
      if (items.length > 0) {
        const scrollTop = parentRef.current!.scrollTop;
        const viewportHeight = parentRef.current!.clientHeight;
        const centerY = scrollTop + viewportHeight / 2;

        const centerItem = items.find((item) => {
          return item.start <= centerY && centerY <= item.end;
        });

        const visibleItem = centerItem || items[0];

        if (visibleItem && visibleItem.index !== currentPage) {
          setCurrentPage(visibleItem.index);
        }
      }
    };

    parentRef.current.addEventListener("scroll", handleScroll);

    const parent = parentRef.current;
    return () => {
      parent?.removeEventListener("scroll", handleScroll);
    };
  }, [virtualizer, currentPage]);

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
    <>
      <div
        ref={parentRef}
        className="h-screen overflow-auto"
        style={{
          contain: "strict",
        }}
      >
        <div
          style={{
            height: `${virtualizer.getTotalSize()}px`,
            width: "100%",
            position: "relative",
          }}
        >
          {virtualizer.getVirtualItems().map((virtualItem) => (
            <div
              key={virtualItem.key}
              data-index={virtualItem.index}
              ref={(el) => {
                if (el) {
                  virtualizer.measureElement(el);
                }
              }}
              style={{
                position: "absolute",
                top: 0,
                left: 0,
                width: "100%",
                minHeight: `${virtualItem.size}px`,
                transform: `translateY(${virtualItem.start}px)`,
              }}
              className="flex justify-center"
            >
              <div className="w-full xl:w-1/2 flex justify-center">
                <Image
                  src={`pages://localhost/${libraryEntry.metafile.source.id}/${fileIndex}/${virtualItem.index}`}
                  alt={`Page ${virtualItem.index + 1}`}
                  className="max-w-full h-auto"
                  style={{ objectFit: "contain" }}
                  height={dimensions[virtualItem.index]?.[1] ?? 1000}
                  width={dimensions[virtualItem.index]?.[0] ?? 500}
                  quality={100}
                  loading={
                    Math.abs(virtualItem.index - currentPage) <=
                    virtualizer.options.overscan
                      ? "eager"
                      : "lazy"
                  }
                />
              </div>
            </div>
          ))}
        </div>
      </div>

      {numPages > 0 && (
        <div className="fixed bottom-2 right-2 text-muted-foreground text-[0.7rem] bg-black/70 backdrop-blur-sm px-2 py-1 rounded shadow-lg">
          {currentPage + 1} / {numPages}
        </div>
      )}
    </>
  );
};
