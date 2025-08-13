import { LibraryEntry } from "@/types/LibraryEntry";
import { invoke } from "@tauri-apps/api/core";
import { EllipsisVertical, Trash } from "lucide-react";
import Image from "next/image";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "./ui/dropdown-menu";

export const LibraryCard = ({
  libraryEntry,
  onDeleteAction,
  setSelectedAction,
}: {
  libraryEntry: LibraryEntry;
  onDeleteAction: (id: string) => void;
  setSelectedAction: (entry: LibraryEntry) => void;
}) => {
  const {
    name,
    metafile: {
      source: { id },
      metadata,
    },
  } = libraryEntry;

  return (
    <div className="flex flex-col items-center justify-between border-1 rounded-xl p-4 w-52 gap-2 hover:bg-muted/80 transition-colors duration-200">
      <div
        className="hover:cursor-pointer"
        onClick={() => setSelectedAction(libraryEntry)}
      >
        {metadata?.cover && (
          <Image
            src={metadata.cover_raw ?? metadata?.cover}
            alt="Cover Image"
            className="rounded"
            style={{ objectFit: "contain" }}
            width={200}
            height={200}
          />
        )}
        <div className="p-2 mt-2 text-center">
          <h1>{name}</h1>
        </div>
      </div>
      <div className="flex justify-end w-full">
        <DropdownMenu>
          <DropdownMenuTrigger className="hover:bg-background hover:cursor-pointer p-1 rounded-full aspect-square transition-colors duration-200">
            <EllipsisVertical />
          </DropdownMenuTrigger>
          <DropdownMenuContent>
            <DropdownMenuItem
              onClick={async () => {
                await invoke("delete", { id });
                onDeleteAction(id);
              }}
            >
              <Trash /> Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  );
};
