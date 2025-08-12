import { LibraryEntry } from "@/types/LibraryEntry";
import Image from "next/image";

export const LibraryCard = ({
  libraryEntry: {
    name,
    metafile: { metadata },
  },
}: {
  libraryEntry: LibraryEntry;
}) => (
  <div className="flex flex-col items-center border-1 rounded-xl p-4 w-52 gap-2">
    {metadata?.cover && (
      <Image
        src={metadata?.cover}
        alt="Cover Image"
        className="rounded"
        style={{ objectFit: "contain" }}
        width={200}
        height={200}
      />
    )}
    <div className="p-2 text-center">
      <h1>{name}</h1>
    </div>
  </div>
);
