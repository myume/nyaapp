"use client";

import { HexColorInput, HexColorPicker } from "react-colorful";
import { zodResolver } from "@hookform/resolvers/zod";
import { useReader } from "../providers/ReaderProvider";
import { z } from "zod";
import { useForm, WatchObserver } from "react-hook-form";
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
import { LibraryEntrySettings, ReaderLayout } from "@/types/LibraryEntry";
import { useEffect, useCallback } from "react";
import { useDebouncedCallback } from "use-debounce";
import { invoke } from "@tauri-apps/api/core";

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
    setReaderContext,
  } = useReader();
  const form = useForm<z.infer<typeof settingsSchema>>({
    resolver: zodResolver(settingsSchema),
    defaultValues: {
      gap: libraryEntry?.metafile.settings?.reader?.gap?.toString() ?? "0",
      background_color:
        libraryEntry?.metafile.settings?.reader?.background_color ?? "#000000",
      layout: libraryEntry?.metafile.settings?.reader?.layout ?? "LongStrip",
    },
  });

  const updateReaderSettings: WatchObserver<z.infer<typeof settingsSchema>> =
    useCallback(
      async (values) => {
        form.trigger();
        if (form.formState.isValid) {
          const settings = {
            ...values,
            gap: parseInt(values.gap!),
          };
          setReaderContext((context) => {
            const updatedContext = { ...context };
            if (updatedContext.libraryEntry?.metafile.settings) {
              updatedContext.libraryEntry.metafile.settings.reader =
                settings as LibraryEntrySettings["reader"];
            }
            return updatedContext;
          });

          await invoke("update_library_entry_settings", {
            id: libraryEntry?.metafile.source.id,
            settings: {
              ...libraryEntry?.metafile.settings,
              reader: {
                ...libraryEntry?.metafile.settings?.reader,
                settings,
              },
            },
          });
        }
      },
      [form, libraryEntry?.metafile, setReaderContext],
    );

  const debouncedUpdate = useDebouncedCallback(updateReaderSettings, 300);

  useEffect(() => {
    const subscription = form.watch(debouncedUpdate);
    return () => {
      subscription.unsubscribe();
      updateReaderSettings(form.getValues(), {});
    };
  }, [form, updateReaderSettings, debouncedUpdate]);

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
                        {layout.replace(/([A-Z])/g, " $1").trim()}
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
