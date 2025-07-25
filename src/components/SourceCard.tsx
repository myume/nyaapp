import { SearchResult } from "@/types/SearchResult";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./ui/card";
import Image from "next/image";
import { ArrowDown, ArrowUp, Book, Check } from "lucide-react";

export const SourceCard = ({
  searchResult: {
    source_media: {
      title,
      size: { unit, size },
      seeders,
      category,
      leechers,
      completed,
      timestamp,
    },
    metadata,
  },
}: {
  searchResult: SearchResult;
}) => {
  return (
    <Card>
      <CardHeader className="h-4">
        <CardDescription>{category}</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col h-full">
        {metadata && metadata.cover ? (
          <Image
            className="flex-1 w-full"
            src={metadata?.cover}
            alt={`${metadata.title} Cover`}
            width={64}
            height={64}
            style={{ objectFit: "contain" }}
          />
        ) : (
          <Book className="flex-1 w-full h-full" size={260} />
        )}
        <CardTitle className="mt-4 mb-2">{title}</CardTitle>
        <CardDescription className="flex gap-1">
          <div className="flex items-center">
            {seeders} <ArrowUp size={16} />
          </div>
          <div className="flex items-center">
            {leechers} <ArrowDown size={16} />
          </div>
          <div className="flex items-center">
            {completed} <Check size={16} />
          </div>
        </CardDescription>
        <CardDescription>
          {size} {unit}
        </CardDescription>
        <CardDescription className="mt-2">
          {new Date(timestamp).toLocaleString()}
        </CardDescription>
      </CardContent>
    </Card>
  );
};
