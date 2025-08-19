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

export type LibraryEntrySettings = {
  reader: {
    gap?: number;
    background_color?: string;
    layout?: ReaderLayout;
  };
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
