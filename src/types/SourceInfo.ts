export type Category = "Manga";

export type FileSize = {
  unit: "MiB" | "GiB";
  size: number;
};

export type MediaInfo = {
  id: string;
  category: Category;
  title: string;
  size: FileSize;
  timestamp: string;
  seeders: number;
  leechers: number;
  completed: number;
};

export type SourceMeta = {
  id: string;
  provider: string;
};
