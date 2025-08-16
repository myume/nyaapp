import { Metadata } from "./Metadata";
import { SourceMeta } from "./SourceInfo";

type Metafile = {
  source: SourceMeta;
  metadata: Metadata | null;
  reading_progress: {
    [filename: string]: number;
  };
};

export type LibraryEntry = {
  name: string;
  metafile: Metafile;
  output_dir: string;
  files: string[];
};
