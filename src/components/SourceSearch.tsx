"use client";

import { useState } from "react";
import { Input } from "./ui/input";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import { optionsToQueryMap } from "@/lib/sourceOptionsToQuery";
import { Button } from "./ui/button";
import { Search } from "lucide-react";

type SourceOption = {
  name: string;
  options: { name: string; value: string }[];
  defaultValue?: string;
};

const sourceToOptions: { [source: string]: SourceOption[] } = {
  Nyaa: [
    {
      name: "Filter",
      options: [
        { name: "No Filter", value: "0" },
        { name: "No Remakes", value: "1" },
        { name: "Trusted Only", value: "2" },
      ],
    },
    {
      name: "Category",
      options: [
        // { name: "All Categories", value: "0_0" },
        // { name: "Anime", value: "1_0" },
        // { name: "Anime - Music Video", value: "1_1" },
        // { name: "Anime - English Translated", value: "1_2" },
        // { name: "Anime - Non-English Translated", value: "1_3" },
        // { name: "Anime - Raw", value: "1_4" },
        // { name: "Audio", value: "2_0" },
        // { name: "Audio - Lossless", value: "2_1" },
        // { name: "Audio - Lossy", value: "2_2" },
        { name: "Literature", value: "3_0" },
        { name: "Literature - English Translated", value: "3_1" },
        { name: "Literature - Non-English Translated", value: "3_2" },
        { name: "Literature - Raw", value: "3_3" },
      ],
      defaultValue: "3_1",
    },
  ],
};

export const SourceSearch = ({
  setQueryAction,
}: {
  setQueryAction: (q: string) => void;
}) => {
  const [source, setSource] = useState("Nyaa");
  const sourceOptions = sourceToOptions[source] ?? [];

  return (
    <form
      className="flex gap-2"
      onSubmit={(e) => {
        e.preventDefault();
        const formData = new FormData(e.currentTarget as HTMLFormElement);
        const data = Object.fromEntries(formData.entries()) as Record<
          string,
          string
        >;

        const converter = optionsToQueryMap[source];
        const queryString = converter(data);
        setQueryAction(queryString);
      }}
    >
      <Select name="source" value={source} onValueChange={setSource}>
        <SelectTrigger className="cursor-pointer">
          <SelectValue placeholder="Source" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectLabel>Sources</SelectLabel>
            <SelectItem value="Nyaa">Nyaa</SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>

      {sourceOptions.map(({ name, options, defaultValue }) => (
        <Select name={name} key={name} defaultValue={defaultValue}>
          <SelectTrigger className="cursor-pointer">
            <SelectValue placeholder={name} />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectLabel>{name}</SelectLabel>
              {options.map(({ name, value }) => (
                <SelectItem
                  className="cursor-pointer"
                  key={value}
                  value={value}
                >
                  {name}
                </SelectItem>
              ))}
            </SelectGroup>
          </SelectContent>
        </Select>
      ))}
      <Input name="query" placeholder="Search..." />
      <Button
        className="cursor-pointer"
        variant="outline"
        size="icon"
        type="submit"
      >
        <Search />
      </Button>
    </form>
  );
};
