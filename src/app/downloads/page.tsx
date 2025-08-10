"use client";

import { DownloadsCard } from "@/components/DownloadsCard";
import { useDownloads } from "@/components/providers/DownloadsProvider";

export default function Downloads() {
  const { downloads } = useDownloads();

  return (
    <div className="space-y-5">
      {Object.values(downloads).map((download) => (
        <DownloadsCard key={download?.id} download={download!} />
      ))}
    </div>
  );
}
