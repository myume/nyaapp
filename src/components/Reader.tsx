import { invoke } from "@tauri-apps/api/core";
import Image from "next/image";
import { useEffect, useRef, useState } from "react";
import { useReader } from "./providers/ReaderProvider";

export const Reader = () => {
  let {
    readerContext: { libraryEntry, fileIndex },
    setReaderContext,
  } = useReader();
  libraryEntry = libraryEntry!;
  fileIndex = fileIndex!;

  const filename = libraryEntry.files[fileIndex];

  const [numPages, setNumPages] = useState(0);
  const [currentPage, setCurrentPage] = useState(
    libraryEntry.metafile.reading_progress[filename] ?? 0,
  );
  const pagesRef = useRef<(HTMLImageElement | null)[]>([]);
  const observer = useRef<IntersectionObserver | null>(null);

  useEffect(() => {
    (async () => {
      const numPages = await invoke<number>("load_cbz", {
        id: libraryEntry.metafile.source.id,
        fileNum: fileIndex,
      });
      setNumPages(numPages);
    })();
  }, [fileIndex, libraryEntry]);

  useEffect(() => {
    const lastReadPage = libraryEntry.metafile.reading_progress[filename] ?? 0;
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
    (async () => {
      await invoke("update_reading_progress", {
        id: libraryEntry.metafile.source.id,
        fileNum: fileIndex,
        updatedPage: currentPage,
      });
    })();
    return () => {
      setReaderContext((context) => {
        const updatedContext = { ...context };
        updatedContext.libraryEntry!.metafile.reading_progress[filename] =
          currentPage;
        return updatedContext;
      });
    };
  }, [currentPage, fileIndex, libraryEntry.metafile.source.id, filename]);

  return (
    <div>
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
            className="w-full"
            style={{ objectFit: "contain" }}
            height={500}
            width={500}
            quality={100}
          />
        ))}
      </div>
      {numPages > 0 && (
        <div className="fixed bottom-2 right-2 text-muted-foreground text-[0.7rem]">
          {currentPage + 1} / {numPages}
        </div>
      )}
    </div>
  );
};
