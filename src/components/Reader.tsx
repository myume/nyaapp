import { LibraryEntry } from "@/types/LibraryEntry";
import { Page } from "@/types/Page";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useVirtualizer } from "@tanstack/react-virtual";
import Image from "next/image";
import { useEffect, useRef, useState } from "react";

export const Reader = ({
  libraryEntry,
  fileIndex,
}: {
  libraryEntry: LibraryEntry;
  fileIndex: number;
}) => {
  const [pages, setPages] = useState<Page[]>([]);
  const parentRef = useRef<HTMLDivElement>(null);

  const rowVirtualizer = useVirtualizer({
    count: pages.length,
    getScrollElement: () => parentRef.current,
    estimateSize: (index) => {
      const page = pages[index];
      if (page && page.width > 0) {
        const containerWidth =
          parentRef.current?.clientWidth || window.innerWidth;
        return (page.height / page.width) * containerWidth;
      }
      return 1200;
    },
    overscan: 5,
  });

  useEffect(() => {
    let unlisten: () => void;

    const setupListener = async () => {
      unlisten = await listen<Page>("page-read", ({ payload: page }) => {
        setPages((pages) => {
          if (pages.find((p) => p.data === page.data)) {
            return pages;
          }
          return [...pages, page];
        });
      });

      await invoke("read_cbz", {
        path: `${libraryEntry.output_dir}/${libraryEntry.files[fileIndex]}`,
      });
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [fileIndex, libraryEntry]);

  return (
    <div ref={parentRef} className="h-[calc(100vh-4rem)] overflow-auto">
      <div
        style={{
          height: `${rowVirtualizer.getTotalSize()}px`,
          width: "100%",
          position: "relative",
        }}
      >
        {rowVirtualizer.getVirtualItems().map((virtualItem) => {
          const page = pages[virtualItem.index];
          return (
            <div
              key={virtualItem.key}
              style={{
                position: "absolute",
                top: 0,
                left: 0,
                width: "100%",
                height: `${virtualItem.size}px`,
                transform: `translateY(${virtualItem.start}px)`,
              }}
            >
              <Image
                src={`data:image/*;base64,${page.data}`}
                alt={`Page ${virtualItem.index + 1}`}
                className="w-full"
                height={page.height}
                width={page.width}
                quality={100}
              />
            </div>
          );
        })}
      </div>
    </div>
  );
};
