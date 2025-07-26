"use client";

import { SourceCard } from "@/components/SourceCard";
import { SearchResult } from "@/types/SearchResult";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function Browse() {
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
  useEffect(() => {
    (async () => {
      const results = await invoke<SearchResult[]>("search", {
        query: "c=3_1",
      });
      setSearchResults(results);
    })();
  }, []);

  return (
    <div className="w-full">
      <div className="p-5 grid md:grid-cols-3 lg:grid-cols-5 gap-5">
        {searchResults.map((result) => (
          <SourceCard key={result.source_info.id} searchResult={result} />
        ))}
      </div>
    </div>
  );
}
