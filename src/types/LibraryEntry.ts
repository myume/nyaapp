import { Metadata } from "./Metadata";
import { SourceMeta } from "./SourceInfo";

type Metafile = {
  source: SourceMeta;
  metadata: Metadata | null;
  reading_progress: {
    [filename: string]:
      | { current_page: number; total_pages: number }
      | undefined;
  };
  settings: LibraryEntrySettings | null;
};

type LibraryEntrySettings = {
  gap: number | null;
  background_color: string | null;
  layout: ReaderLayout | null;
};

export enum ReaderLayout {
  LongStrip = "LongStrip",
  SinglePage = "SinglePage",
  DoublePage = "DoublePage",
}

export type LibraryEntry = {
  name: string;
  metafile: Metafile;
  output_dir: string;
  files: string[];
};
