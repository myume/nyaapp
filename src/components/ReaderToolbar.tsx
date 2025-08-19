"use client";

import { ChevronLeft, ChevronRight } from "lucide-react";
import { useReader } from "./providers/ReaderProvider";
import { Button } from "./ui/button";

export const ReaderToolbar = ({
  currentPage,
  numPages,
  setCurrentPageAction,
}: {
  currentPage: number;
  numPages: number;
  setCurrentPageAction: (i: number) => void;
}) => {
  const { readerContext, setReaderContext } = useReader();

  return (
    <div className="bg-background/80 px-4 py-2 flex items-center justify-between gap-5 text-sm">
      <h1>{readerContext.libraryEntry?.files[readerContext.fileIndex ?? 0]}</h1>
      <div className="flex gap-5">
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            disabled={currentPage <= 0}
            onClick={() => setCurrentPageAction(Math.max(currentPage - 1, 0))}
          >
            <ChevronLeft />
          </Button>
          <div className="flex flex-col justify-center items-center text-xs">
            <h2>Page</h2>
            <h4>
              {currentPage + 1} / {numPages}
            </h4>
          </div>
          <Button
            variant="outline"
            disabled={currentPage >= numPages - 1}
            onClick={() =>
              setCurrentPageAction(Math.min(currentPage + 1, numPages - 1))
            }
          >
            <ChevronRight />
          </Button>
        </div>
        <div className="flex items-center gap-2">
          <Button
            disabled={readerContext.fileIndex === 0}
            variant="outline"
            onClick={() =>
              setReaderContext((context) => ({
                ...context,
                fileIndex:
                  context.fileIndex !== undefined
                    ? Math.max(context.fileIndex - 1, 0)
                    : 0,
              }))
            }
          >
            <ChevronLeft />
          </Button>
          <div className="flex flex-col justify-center items-center text-xs">
            <h2>File</h2>
            <h4>
              {(readerContext.fileIndex ?? 0) + 1} /{" "}
              {readerContext.libraryEntry?.files.length}
            </h4>
          </div>
          <Button
            disabled={
              readerContext.libraryEntry?.files.length !== undefined &&
              readerContext.fileIndex ===
                readerContext.libraryEntry?.files.length - 1
            }
            variant="outline"
            onClick={() =>
              setReaderContext((context) => ({
                ...context,
                fileIndex:
                  context.fileIndex !== undefined
                    ? Math.min(
                        context.fileIndex + 1,
                        context.libraryEntry?.files.length !== undefined
                          ? context.libraryEntry.files.length - 1
                          : 0,
                      )
                    : 0,
              }))
            }
          >
            <ChevronRight />
          </Button>
        </div>
      </div>
    </div>
  );
};

