import { LibraryEntry } from "@/types/LibraryEntry";
import Image from "next/image";

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
  return (
    <div className="relative flex justify-around items-center h-full">
      <div
        className="absolute left-0 top-0 h-screen w-1/2"
        onClick={() => {
          setCurrentPage(Math.max(currentPage - columns, 0));
        }}
      />
      <div
        className="absolute right-0 top-0 h-screen w-1/2"
        onClick={() => {
          setCurrentPage(Math.min(currentPage + columns, numPages - columns));
        }}
      />
      <div
        style={{
          display: "grid",
          gridTemplateColumns: `repeat(${columns}, 1fr)`,
        }}
      >
        {Array.from(
          { length: Math.min(columns, numPages - currentPage) },
          (_, i) => {
            const pageIndex = currentPage + i;
            return (
              <Image
                key={pageIndex}
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
                priority
              />
            );
          },
        )}
      </div>
    </div>
  );
};
