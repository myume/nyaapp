import { Metadata } from "./Metadata";
import { SourceInfo } from "./SourceInfo";

export type SearchResult = {
  source_info: SourceInfo;
  metadata?: Metadata;
};
