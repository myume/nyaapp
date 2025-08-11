"use client";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import { toast } from "sonner";

export type DownloadInfo = {
  id: string;
  name: string | null;
  state: "initializing" | "live" | "paused" | "error";
  progress_bytes: number;
  uploaded_bytes: number;
  total_bytes: number;
  finished: boolean;
  upload_speed: number | null;
  download_speed: number | null;
  remaining_time: string | null;
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
    (async () => {
      let results = await invoke<DownloadInfo[]>("list_torrents");
      setDownloads(
        results.reduce((acc, curr) => {
          return { ...acc, [curr.id]: curr };
        }, {}),
      );
    })();

    listen<string[]>("download-started", ({ payload }) => {
      const [id, name] = payload;
      toast(`Downloading: ${name}`);
      setDownloads((downloads) => ({
        ...downloads,
        [id]: {
          id,
          name,
          state: "initializing",
          progress_bytes: 0,
          uploaded_bytes: 0,
          total_bytes: 0,
          finished: false,
          upload_speed: null,
          download_speed: null,
          remaining_time: null,
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
      setDownloads((downloads) => {
        toast(`Finished downloading: ${downloads[id].name}`);
        return {
          ...downloads,
          [id]: {
            ...downloads[id],
            finished: true,
            download_speed: null,
            upload_speed: null,
          },
        };
      });
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
