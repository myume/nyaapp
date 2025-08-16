"use client";

import { LibraryEntry } from "@/types/LibraryEntry";
import {
  createContext,
  Dispatch,
  ReactNode,
  SetStateAction,
  useContext,
  useState,
} from "react";

export type ReaderContext = {
  fileIndex?: number;
  libraryEntry?: LibraryEntry;
};

type ReaderContextType = {
  readerContext: ReaderContext;
  setReaderContext: Dispatch<SetStateAction<ReaderContext>>;
};

const ReaderContext = createContext<ReaderContextType | undefined>(undefined);

export function ReaderProvider({ children }: { children: ReactNode }) {
  const [readerContext, setReaderContext] = useState<ReaderContext>({});

  return (
    <ReaderContext.Provider value={{ readerContext, setReaderContext }}>
      {children}
    </ReaderContext.Provider>
  );
}

export function useReader() {
  const readerContext = useContext(ReaderContext);
  if (readerContext === undefined) {
    throw new Error("useDownloads must be used within a ReaderProvider");
  }

  return readerContext;
}
