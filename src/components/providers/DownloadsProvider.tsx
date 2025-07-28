"use client";

import { listen } from "@tauri-apps/api/event";
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import { toast } from "sonner";

type DownloadInfo = {
  id: string;
  state: "initializing" | "live" | "paused" | "error";
  progress_bytes: number;
  uploaded_bytes: number;
  total_bytes: number;
  finished: boolean;
};

type DownloadsContextType = {
  downloads: Record<string, DownloadInfo | undefined>;
  setDownloads: (downloads: Record<string, DownloadInfo>) => void;
};

const DownloadsContext = createContext<DownloadsContextType | undefined>(
  undefined,
);

export function DownloadsProvider({ children }: { children: ReactNode }) {
  const [downloads, setDownloads] = useState<Record<string, DownloadInfo>>({});

  useEffect(() => {
    listen<string>("download-started", ({ payload: id }) => {
      toast(`Started download for ${id}`);
      setDownloads((downloads) => ({
        ...downloads,
        [id]: {
          id,
          state: "initializing",
          progress_bytes: 0,
          uploaded_bytes: 0,
          total_bytes: 0,
          finished: false,
        },
      }));
    });

    listen<DownloadInfo>("download-progress", ({ payload: downloadInfo }) => {
      setDownloads((downloads) => ({
        ...downloads,
        [downloadInfo.id]: downloadInfo,
      }));
    });

    listen<string>("download-completed", ({ payload: id }) => {
      toast(`Finished download for ${id}`);
      setDownloads((downloads) => ({
        ...downloads,
        [id]: {
          ...downloads[id],
          finished: true,
        },
      }));
    });
  }, []);

  return (
    <DownloadsContext.Provider value={{ downloads, setDownloads }}>
      {children}
    </DownloadsContext.Provider>
  );
}
export function useDownloads() {
  const downloads = useContext(DownloadsContext);
  if (downloads === undefined) {
    throw new Error("useDownloads must be used within a DownloadsProvider");
  }

  return downloads;
}
