"use client";

import { invoke } from "@tauri-apps/api/core";
import { info, error as logError } from "@tauri-apps/plugin-log";

import { useEffect, useRef, useState } from "react";
import { LongStripLayout } from "./LongStripLayout";
import { VirtuosoHandle } from "react-virtuoso";

import { useReader } from "../providers/ReaderProvider";
import { ReaderToolbar } from "./ReaderToolbar";
import { useDebouncedCallback } from "use-debounce";
import { ReaderLayout } from "@/types/LibraryEntry";
import { PagedLayout } from "./PagedLayout";
import { Spinner } from "../ui/spinner";
import { Button } from "../ui/button";
import { useRouter } from "next/navigation";

export const Reader = () => {
  const { readerContext, setReaderContext } = useReader();
  const router = useRouter();
  let { libraryEntry, fileIndex } = readerContext;
  libraryEntry = libraryEntry!;
  fileIndex = fileIndex!;

  const filename = libraryEntry.files[fileIndex];

  const [loading, setLoading] = useState(false);
  const [numPages, setNumPages] = useState(0);
  const [error, setError] = useState<string | undefined>(undefined);
  const [currentPage, setCurrentPage] = useState(
    libraryEntry.metafile.reading_progress[filename]?.current_page ?? 0,
  );
  const [dimensions, setDimensions] = useState<[number, number][]>([]);
  const virtuoso = useRef<VirtuosoHandle | null>(null);
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
    const loadPages = async () => {
      setLoading(true);
      setError(undefined);
      try {
        info("Loading pages...");
        const numPages = await invoke<number>("load_cbz", {
          id: libraryEntry.metafile.source.id,
          fileNum: fileIndex,
        });
        setNumPages(numPages);
        setCurrentPage(
          libraryEntry.metafile.reading_progress[filename]?.current_page ?? 0,
        );
        const dimensions = await invoke<[number, number][]>("get_dimensions", {
          id: libraryEntry.metafile.source.id,
          fileNum: fileIndex,
        });
        setDimensions(dimensions);
      } catch (e) {
        setError(`Failed to load pages: ${e}`);
      } finally {
        setLoading(false);
      }
    };
    loadPages();
  }, [fileIndex, libraryEntry, setLoading, filename]);

  const updateReadingProgress = useDebouncedCallback(async () => {
    try {
      info("Restoring reading progress...");
      await invoke("update_reading_progress", {
        id: libraryEntry.metafile.source.id,
        fileNum: fileIndex,
        updatedPage: currentPage,
      });
    } catch (e) {
      logError(e as string);
    }
  }, 500);

  useEffect(() => {
    updateReadingProgress();

    return () => {
      setReaderContext((context) => {
        const updatedContext = { ...context };
        if (updatedContext.libraryEntry) {
          updatedContext.libraryEntry.metafile.reading_progress[filename] = {
            current_page: currentPage,
            total_pages: numPages,
          };
        }
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
    updateReadingProgress,
  ]);

  if (error) {
    return (
      <div className="flex flex-col items-center justify-center w-full h-screen gap-4">
        <p className="text-red-500">{error}</p>
        <Button
          variant="secondary"
          onClick={() => {
            router.replace("/library");
            setReaderContext({});
          }}
        >
          Back to Library
        </Button>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center w-full h-screen">
        <Spinner size="large" />
      </div>
    );
  }

  let viewer;
  let pageOffset = 1;
  switch (libraryEntry.metafile.settings?.reader.layout) {
    case ReaderLayout.SinglePage:
      viewer = (
        <PagedLayout
          numPages={numPages}
          currentPage={currentPage}
          libraryEntry={libraryEntry}
          fileIndex={fileIndex}
          dimensions={dimensions}
          setCurrentPage={setCurrentPage}
          columns={1}
        />
      );
      pageOffset = 1;
      break;
    case ReaderLayout.DoublePage:
      viewer = (
        <PagedLayout
          numPages={numPages}
          currentPage={currentPage}
          libraryEntry={libraryEntry}
          fileIndex={fileIndex}
          dimensions={dimensions}
          setCurrentPage={setCurrentPage}
          columns={2}
        />
      );
      pageOffset = 2;
      break;
    case ReaderLayout.LongStrip:
    default:
      viewer = (
        <LongStripLayout
          numPages={numPages}
          currentPage={currentPage}
          virtuoso={virtuoso}
          libraryEntry={libraryEntry}
          fileIndex={fileIndex}
          dimensions={dimensions}
          observer={observer}
        />
      );
      pageOffset = 1;
      break;
  }

  return (
    <div
      className="relative"
      style={{
        background:
          libraryEntry.metafile.settings?.reader.background_color ?? "#000000",
      }}
    >
      <div className="absolute top-0 w-full z-10 opacity-0 hover:opacity-100 transition-opacity duration-300 has-[input:focus]:opacity-100 has-[[data-state=open]]:opacity-100">
        <ReaderToolbar
          currentPage={currentPage}
          numPages={numPages}
          nextPageOffset={pageOffset}
          setCurrentPageAction={(page) => {
            setCurrentPage(page);
            virtuoso.current?.scrollToIndex({
              index: page,
              behavior: "smooth",
            });
          }}
        />
      </div>
      <div className="h-screen">{numPages > 0 && viewer}</div>
      {numPages > 0 && (
        <div className="absolute bottom-2 right-2 text-muted-foreground text-[0.7rem]">
          {currentPage + 1} / {numPages}
        </div>
      )}
    </div>
  );
};
