import { Metadata } from "./Metadata";
import { SourceMeta } from "./SourceInfo";

type Metafile = {
  source: SourceMeta;
};

export type LibraryEntry = {
  metafile: Metafile;
  output_dir: string;
  files: string[];
  metadata: Metadata | null;
};
