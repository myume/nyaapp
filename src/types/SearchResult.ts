import { Metadata } from "./Metadata";
import { PaginationInfo } from "./PaginationInfo";
import { SourceMedia } from "./SourceInfo";

export type SearchResult = {
  source_media: SourceMedia;
  metadata?: Metadata;
};

export type SearchResponse = {
  search_results: SearchResult[];
  pagination: PaginationInfo;
};
