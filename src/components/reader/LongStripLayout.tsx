import { Virtuoso, VirtuosoHandle } from "react-virtuoso";
import Image from "next/image";
import { RefObject, useEffect, useState } from "react";
import { LibraryEntry } from "@/types/LibraryEntry";

interface LongStripLayoutProps {
  numPages: number;
  currentPage: number;
  virtuoso: RefObject<VirtuosoHandle | null>;
  libraryEntry: LibraryEntry;
  fileIndex: number;
  dimensions: [number, number][];
  observer: RefObject<IntersectionObserver | null>;
}

export function LongStripLayout({
  numPages,
  currentPage,
  virtuoso,
  libraryEntry,
  fileIndex,
  dimensions,
  observer,
}: LongStripLayoutProps) {
  const [windowWidth, setWindowWidth] = useState(0);

  useEffect(() => {
    const handleResize = () => {
      setWindowWidth(window.innerWidth);
      virtuoso.current?.scrollToIndex(currentPage);
    };
    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, [currentPage, virtuoso]);

  return (
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
            style={{
              objectFit: "contain",
              paddingBottom:
                i !== numPages - 1
                  ? (libraryEntry.metafile.settings?.reader.gap ?? 0)
                  : 0,
            }}
            className="m-auto w-full xl:w-1/2"
            height={dimensions[i]?.[1] || 1000}
            width={dimensions[i]?.[0] || 500}
            quality={100}
            priority
          />
        </div>
      )}
    />
  );
}
