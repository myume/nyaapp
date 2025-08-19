"use client";

import { HexColorInput, HexColorPicker } from "react-colorful";
import { zodResolver } from "@hookform/resolvers/zod";
import { useReader } from "../providers/ReaderProvider";
import { z } from "zod";
import { useForm } from "react-hook-form";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "../ui/form";
import { Input } from "../ui/input";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";
import { ReaderLayout } from "@/types/LibraryEntry";
import { useEffect } from "react";
import { useDebouncedCallback } from "use-debounce";

const settingsSchema = z.object({
  gap: z.string().regex(/^\d+$/, "Must be a valid number"),
  background_color: z
    .string()
    .regex(/^#[A-Fa-f0-9]{6}$/, "Must be a valid 6-digit hex color"),
  layout: z.enum(["LongStrip", "SinglePage", "DoublePage"]),
});

export const ReaderMenu = () => {
  const {
    readerContext: { libraryEntry },
  } = useReader();
  const form = useForm<z.infer<typeof settingsSchema>>({
    resolver: zodResolver(settingsSchema),
    defaultValues: {
      gap: libraryEntry?.metafile.settings?.gap?.toString() ?? "0",
      background_color:
        libraryEntry?.metafile.settings?.background_color ?? "#000000",
      layout: libraryEntry?.metafile.settings?.layout ?? "LongStrip",
    },
  });

  const debouncedUpdate = useDebouncedCallback((values) => {
    if (form.formState.isValid) {
      console.log(values);
    }
  }, 300);

  useEffect(() => {
    const subscription = form.watch(debouncedUpdate);
    return () => subscription.unsubscribe();
  }, [form]);

  return (
    <Form {...form}>
      <form className="space-y-8">
        <FormField
          control={form.control}
          name="layout"
          render={({ field }) => (
            <FormItem className="flex justify-between">
              <FormLabel>Layout</FormLabel>
              <Select onValueChange={field.onChange} defaultValue={field.value}>
                <FormControl>
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectGroup>
                    {Object.values(ReaderLayout).map((layout) => (
                      <SelectItem key={layout} value={layout}>
                        {layout}
                      </SelectItem>
                    ))}
                  </SelectGroup>
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="gap"
          render={({ field }) => (
            <FormItem>
              <div className="flex justify-between items-center">
                <div className="flex-1 space-y-1">
                  <FormLabel>Gap</FormLabel>
                  <FormDescription className="text-xs">
                    Gap between images in longstrip layout
                  </FormDescription>
                </div>
                <FormControl className="basis-1/6">
                  <Input type="number" min={0} className="w-auto" {...field} />
                </FormControl>
              </div>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="background_color"
          render={({ field }) => (
            <FormItem className="flex justify-between items-start">
              <FormLabel className="whitespace-nowrap">
                Background Color
              </FormLabel>
              <FormControl>
                <div className="space-y-2">
                  <HexColorPicker color={field.value} {...field} />
                  <HexColorInput
                    className="text-center"
                    color={field.value}
                    {...field}
                    prefixed
                  />
                </div>
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
      </form>
    </Form>
  );
};
