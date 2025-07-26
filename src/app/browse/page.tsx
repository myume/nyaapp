"use client";

import { SourceCard } from "@/components/SourceCard";
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";
import { Spinner } from "@/components/ui/spinner";
import { PaginationInfo } from "@/types/PaginationInfo";
import { SearchResponse, SearchResult } from "@/types/SearchResult";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function Browse() {
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(1);
  const [paginationInfo, setPaginationInfo] = useState<PaginationInfo>();

  const search = async (query: string) => {
    setLoading(true);
    const { search_results: results, pagination } =
      await invoke<SearchResponse>("search", {
        query,
      });
    setSearchResults(results);
    setPaginationInfo(pagination);
    setLoading(false);
  };

  useEffect(() => {
    search(`c=3_1&p=${page}`);
  }, [page]);

  // shoutout gemini for generating this.
  const renderPagination = () => {
    if (!paginationInfo) return null;

    const { min_page, max_page } = paginationInfo;
    const pages = [];
    const totalPages = max_page - min_page + 1;

    if (totalPages <= 7) {
      for (let i = min_page; i <= max_page; i++) {
        pages.push(i);
      }
    } else {
      pages.push(min_page);
      if (page > min_page + 2) {
        pages.push("...");
      }

      let start = Math.max(min_page + 1, page - 1);
      let end = Math.min(max_page - 1, page + 1);

      if (page <= min_page + 2) {
        start = min_page + 1;
        end = min_page + 3;
      }

      if (page >= max_page - 2) {
        start = max_page - 3;
        end = max_page - 1;
      }

      for (let i = start; i <= end; i++) {
        pages.push(i);
      }

      if (page < max_page - 2) {
        pages.push("...");
      }
      pages.push(max_page);
    }

    return pages.map((p, index) => (
      <PaginationItem key={index}>
        {p === "..." ? (
          <PaginationEllipsis />
        ) : (
          <PaginationLink
            className="cursor-pointer"
            isActive={p === page}
            onClick={() => setPage(p as number)}
          >
            {p}
          </PaginationLink>
        )}
      </PaginationItem>
    ));
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center w-full h-[90vh]">
        <Spinner size="large" />
      </div>
    );
  }

  return (
    <div className="w-full h-full">
      <div className="p-5 grid md:grid-cols-3 lg:grid-cols-5 gap-5">
        {searchResults.map((result) => (
          <SourceCard key={result.source_media.id} searchResult={result} />
        ))}
      </div>
      <Pagination>
        <PaginationContent>
          {paginationInfo?.has_prev && (
            <PaginationItem>
              <PaginationPrevious
                className="cursor-pointer"
                onClick={() => setPage((page) => Math.max(page - 1, 1))}
              />
            </PaginationItem>
          )}
          {renderPagination()}
          {paginationInfo?.has_next && (
            <PaginationItem>
              <PaginationNext
                className="cursor-pointer"
                onClick={() => setPage((page) => page + 1)}
              />
            </PaginationItem>
          )}
        </PaginationContent>
      </Pagination>
    </div>
  );
}
