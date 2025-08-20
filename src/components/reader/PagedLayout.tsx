import { LibraryEntry } from "@/types/LibraryEntry";
import Image from "next/image";
import { useCallback, useEffect, useRef } from "react";
import { useDebouncedCallback } from "use-debounce";

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
  const containerRef = useRef<HTMLDivElement | null>(null);

  const prevPage = useCallback(() => {
    setCurrentPage(Math.max(currentPage - columns, columns - 1));
  }, [currentPage, columns]);

  const nextPage = useCallback(() => {
    setCurrentPage(Math.min(currentPage + columns, numPages - 1));
  }, [currentPage, columns]);

  const scrollToNavigate = useDebouncedCallback((event) => {
    event.preventDefault();

    if (event.deltaY < 0) {
      prevPage();
    } else {
      nextPage();
    }
  }, 100);

  useEffect(() => {
    containerRef.current?.addEventListener("wheel", scrollToNavigate, {
      passive: false,
    });
    return () => {
      containerRef.current?.removeEventListener("wheel", scrollToNavigate);
    };
  }, [containerRef, prevPage, nextPage]);

  useEffect(() => {
    const nextMultiple = Math.min(
      Math.floor(currentPage / columns) * columns + columns - 1,
      numPages - 1,
    );
    if (nextMultiple !== currentPage) {
      setCurrentPage(nextMultiple);
    }
  }, [currentPage, columns, setCurrentPage]);

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
              <div key={i} className="flex justify-center items-center">
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
                />
              </div>
            );
          },
        ).reverse()}
      </div>
    </div>
  );
};
