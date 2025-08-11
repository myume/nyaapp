import { Metadata } from "./Metadata";
import { PaginationInfo } from "./PaginationInfo";
import { MediaInfo } from "./SourceInfo";

export type SearchResult = {
  media_info: MediaInfo;
  metadata?: Metadata;
};

export type SearchResponse = {
  search_results: SearchResult[];
  pagination: PaginationInfo;
};
