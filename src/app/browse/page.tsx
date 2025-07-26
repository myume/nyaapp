"use client";

import { SourceCard } from "@/components/SourceCard";
import { Spinner } from "@/components/ui/spinner";
import { SearchResult } from "@/types/SearchResult";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function Browse() {
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(true);

  const search = async (query: string) => {
    setLoading(true);
    const results = await invoke<SearchResult[]>("search", {
      query,
    });
    setSearchResults(results);
    setLoading(false);
  };

  useEffect(() => {
    search("c=3_1");
  }, []);

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
          <SourceCard key={result.source_info.id} searchResult={result} />
        ))}
      </div>
    </div>
  );
}
