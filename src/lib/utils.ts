import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const bytesToString = (bytes: number) => {
  const gib = bytes / Math.pow(2, 30);
  const mib = bytes / Math.pow(2, 20);
  if (gib > 1) {
    return `${gib.toFixed(1)} GiB`;
  }

  if (mib > 1) {
    return `${mib.toFixed(1)} MiB`;
  }

  return `${bytes} B`;
};
