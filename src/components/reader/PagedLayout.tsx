import { LibraryEntry } from "@/types/LibraryEntry";
import Image from "next/image";
import { useCallback, useEffect, useRef } from "react";
import { useDebouncedCallback } from "use-debounce";
import { useReader } from "../providers/ReaderProvider";

interface PagedLayoutProps {
  numPages: number;
  currentPage: number;
  columns: number;
  libraryEntry: LibraryEntry;
  fileIndex: number;
  dimensions: [number, number][];
  setCurrentPage: (page: number) => void;
}

export const PagedLayout = ({
  numPages,
  currentPage,
  columns,
  libraryEntry,
  fileIndex,
  dimensions,
  setCurrentPage,
}: PagedLayoutProps) => {
  const { setReaderContext } = useReader();
  const containerRef = useRef<HTMLDivElement | null>(null);

  const prevPage = useCallback(() => {
    if (currentPage - columns < columns - 1) {
      setReaderContext((context) => ({
        ...context,
        fileIndex: Math.max(fileIndex - 1, 0),
      }));
    } else {
      setCurrentPage(Math.max(currentPage - columns, columns - 1));
    }
  }, [currentPage, columns, setCurrentPage]);

  const nextPage = useCallback(() => {
    if (currentPage === numPages - 1) {
      setReaderContext((context) => ({
        ...context,
        fileIndex: Math.min(fileIndex + 1, libraryEntry.files.length - 1),
      }));
    } else {
      setCurrentPage(Math.min(currentPage + columns, numPages - 1));
    }
  }, [currentPage, columns, setCurrentPage, numPages]);

  const scrollToNavigate = useDebouncedCallback((event) => {
    if (event.deltaY < 0) {
      prevPage();
    } else {
      nextPage();
    }
  }, 100);

  const handleKeybinds = useCallback(
    (e: KeyboardEvent) => {
      switch (e.key) {
        case "ArrowDown":
        case "j":
          setReaderContext((context) => ({
            ...context,
            fileIndex: Math.min(
              (context.fileIndex ?? 0) + 1,
              libraryEntry.files.length - 1,
            ),
          }));
          break;
        case "ArrowUp":
        case "k":
          setReaderContext((context) => ({
            ...context,
            fileIndex: Math.max((context.fileIndex ?? 0) - 1, 0),
          }));
          break;
        case "ArrowRight":
        case "l":
          nextPage();
          break;
        case "ArrowLeft":
        case "h":
          prevPage();
          break;
      }
    },
    [nextPage, prevPage, setReaderContext, libraryEntry],
  );

  useEffect(() => {
    const ref = containerRef.current;
    ref?.addEventListener("wheel", scrollToNavigate, {
      passive: true,
    });
    window.addEventListener("keydown", handleKeybinds);
    return () => {
      ref?.removeEventListener("wheel", scrollToNavigate);
      window.removeEventListener("keydown", handleKeybinds);
    };
  }, [containerRef, scrollToNavigate, handleKeybinds]);

  useEffect(() => {
    const nextMultiple = Math.min(
      Math.floor(currentPage / columns) * columns + columns - 1,
      numPages - 1,
    );
    if (nextMultiple !== currentPage) {
      setCurrentPage(nextMultiple);
    }
  }, [currentPage, columns, setCurrentPage, numPages]);

  return (
    <div
      ref={containerRef}
      className="relative flex justify-around items-center h-full"
    >
      <div
        className="absolute left-0 top-0 h-screen w-1/2"
        onClick={prevPage}
      />
      <div
        className="absolute right-0 top-0 h-screen w-1/2"
        onClick={nextPage}
      />
      <div
        style={{
          display: "grid",
          gridTemplateColumns: `repeat(${columns}, 1fr)`,
        }}
      >
        {Array.from(
          {
            length: Math.min(columns, numPages - currentPage),
          },
          (_, i) => {
            const pageIndex = currentPage - i;
            return (
              <div key={`${i}-${pageIndex}`} className="flex items-center">
                <Image
                  src={`pages://localhost/${libraryEntry.metafile.source.id}/${fileIndex}/${pageIndex}`}
                  alt={`Page ${pageIndex + 1}`}
                  style={{
                    objectFit: "contain",
                    maxHeight: "100vh",
                    maxWidth: "100%",
                  }}
                  className="h-auto w-auto"
                  height={dimensions[pageIndex]?.[1] || 1000}
                  width={dimensions[pageIndex]?.[0] || 500}
                  quality={100}
                  onLoad={(e) => {
                    e.currentTarget.hidden = false;
                  }}
                  onError={(e) => {
                    e.currentTarget.hidden = true;
                  }}
                />
              </div>
            );
          },
        ).reverse()}
      </div>
    </div>
  );
};
