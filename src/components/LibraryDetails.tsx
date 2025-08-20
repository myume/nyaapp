import { LibraryEntry } from "@/types/LibraryEntry";
import { Separator } from "./ui/separator";
import Image from "next/image";
import { cn } from "@/lib/utils";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "./ui/dropdown-menu";
import { EllipsisVertical, Eraser, Eye } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

export const LibraryDetails = ({
  libraryEntry: {
    name,
    files,
    metafile: {
      metadata,
      reading_progress,
      source: { id },
    },
  },
  setFileIndex,
  fetchLibrary,
}: {
  libraryEntry: LibraryEntry;
  setFileIndex: (fileIndex: number) => void;
  fetchLibrary: () => void;
}) => {
  return (
    <div className="space-y-5 p-2">
      <div className="flex flex-col sm:flex-row justify-center gap-5 items-start">
        {metadata?.cover && (
          <Image
            src={metadata?.cover_raw ?? metadata?.cover}
            alt="Cover Image"
            className="rounded m-auto sm:m-0"
            style={{ objectFit: "contain" }}
            width={300}
            height={300}
          />
        )}
        <div className="space-y-5">
          <div>
            <h1 className="font-bold text-xl mb-4">
              {metadata?.title ?? name}
            </h1>
            <h2>Year: {metadata?.year}</h2>
            <h2>Status: {metadata?.status}</h2>
            <h2>Authors: {metadata?.authors?.join(", ")}</h2>
            <h2>Artists: {metadata?.artists?.join(", ")}</h2>
          </div>
          <Separator />
          <p
            className="overflow-y-auto max-h-80 break-words"
            dangerouslySetInnerHTML={{ __html: metadata?.description ?? "" }}
          />
        </div>
      </div>
      <Separator />
      <div className="space-y-5">
        <h2 className="text-2xl font-bold mb-5">Files</h2>
        <ul className="space-y-2 max-h-[50dvh] overflow-auto">
          {files.map((file, i) => {
            const progress = reading_progress[file];
            return (
              <li
                className={cn(
                  "flex w-full justify-between items-center hover:bg-muted/80 p-2 rounded transition-colors duration-200 hover:cursor-pointer",
                  progress &&
                    progress?.current_page + 1 === progress?.total_pages &&
                    "text-muted-foreground/60",
                )}
                key={file}
              >
                <div
                  className="flex-1 flex gap-4 items-center"
                  onClick={() => setFileIndex(i)}
                >
                  <h2>{file}</h2>
                  <h4>
                    {progress && (
                      <span className="text-muted-foreground/60 text-xs">
                        Page {progress.current_page + 1} /{" "}
                        {progress.total_pages}
                      </span>
                    )}
                  </h4>
                </div>
                <DropdownMenu>
                  <DropdownMenuTrigger className="hover:bg-background hover:cursor-pointer p-1 rounded-full aspect-square transition-colors duration-200">
                    <EllipsisVertical className="opacity-80" />
                  </DropdownMenuTrigger>
                  <DropdownMenuContent>
                    <DropdownMenuItem
                      onClick={async () => {
                        await invoke("mark_as_read", { id, fileNum: i });
                        fetchLibrary();
                      }}
                    >
                      <Eye />
                      Mark as Read
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      onClick={async () => {
                        await invoke("clear_reading_progress", {
                          id,
                          fileNum: i,
                        });
                        fetchLibrary();
                      }}
                    >
                      <Eraser />
                      Clear Reading Progress
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </li>
            );
          })}
        </ul>
      </div>
    </div>
  );
};
