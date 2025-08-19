"use client";

import { useEffect, useState } from "react";
import { ChevronLeft, ChevronRight, EllipsisVertical } from "lucide-react";
import { useReader } from "../providers/ReaderProvider";
import { Button } from "../ui/button";
import { Dialog, DialogContent, DialogTrigger } from "../ui/dialog";
import { ReaderMenu } from "./ReaderMenu";
import { DialogTitle } from "@radix-ui/react-dialog";

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
  const [pageInput, setPageInput] = useState((currentPage + 1).toString());

  useEffect(() => {
    setPageInput((currentPage + 1).toString());
  }, [currentPage]);

  const [fileInput, setFileInput] = useState(
    ((readerContext.fileIndex ?? 0) + 1).toString(),
  );

  useEffect(() => {
    setFileInput(((readerContext.fileIndex ?? 0) + 1).toString());
  }, [readerContext.fileIndex]);

  return (
    <div className="bg-background/80 px-4 py-2 flex items-center justify-between gap-5 text-xs">
      <h1 className="flex-1 truncate overflow-ellipsis">
        {readerContext.libraryEntry?.files[readerContext.fileIndex ?? 0]}
      </h1>
      <div className="flex gap-4">
        <div className="flex items-center justify-center gap-2">
          <Button
            variant="outline"
            disabled={currentPage <= 0}
            onClick={() => setCurrentPageAction(Math.max(currentPage - 1, 0))}
          >
            <ChevronLeft />
          </Button>
          <div className="flex flex-col justify-center items-center text-xs whitespace-nowrap">
            <h2>Page</h2>
            <h4>
              <input
                type="number"
                value={pageInput}
                onChange={(e) => setPageInput(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    const newPage = parseInt(e.currentTarget.value) - 1;
                    if (!isNaN(newPage) && newPage >= 0 && newPage < numPages) {
                      setCurrentPageAction(newPage);
                    } else {
                      setPageInput((currentPage + 1).toString());
                    }
                  }
                }}
                className="inline text-center [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                min={1}
                max={numPages}
              />
              / {numPages}
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
        <div className="flex items-center justify-end gap-2">
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
          <div className="flex flex-col justify-center items-center text-xs whitespace-nowrap">
            <h2>File</h2>
            <h4>
              <input
                type="number"
                value={fileInput}
                onChange={(e) => setFileInput(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    const newFileIndex = parseInt(e.currentTarget.value) - 1;
                    const numFiles =
                      readerContext.libraryEntry?.files.length ?? 0;
                    if (
                      !isNaN(newFileIndex) &&
                      newFileIndex >= 0 &&
                      newFileIndex < numFiles
                    ) {
                      setReaderContext((context) => ({
                        ...context,
                        fileIndex: newFileIndex,
                      }));
                    } else {
                      setFileInput(
                        ((readerContext.fileIndex ?? 0) + 1).toString(),
                      );
                    }
                  }
                }}
                className="text-center [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                min={1}
                max={readerContext.libraryEntry?.files.length ?? 0}
              />
              / {readerContext.libraryEntry?.files.length}
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
      <Dialog>
        <DialogTrigger asChild>
          <EllipsisVertical size={16} />
        </DialogTrigger>
        <DialogContent>
          <DialogTitle className="font-bold">Settings</DialogTitle>
          <ReaderMenu />
        </DialogContent>
      </Dialog>
    </div>
  );
};
