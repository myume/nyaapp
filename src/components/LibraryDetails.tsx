import { LibraryEntry } from "@/types/LibraryEntry";
import { Separator } from "./ui/separator";
import Image from "next/image";

export const LibraryDetails = ({
  libraryEntry: {
    name,
    files,
    metafile: { metadata },
  },
  setFileIndex,
}: {
  libraryEntry: LibraryEntry;
  setFileIndex: (fileIndex: number) => void;
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
          {files.map((file, i) => (
            <li
              className="hover:bg-muted/80 p-2 rounded w-fit transition-colors duration-200 hover:cursor-pointer"
              onClick={() => setFileIndex(i)}
              key={file}
            >
              {file}
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};
