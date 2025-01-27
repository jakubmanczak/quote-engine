import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const qfetch = async (
  path: string | URL | globalThis.Request,
  init?: RequestInit,
) => {
  const input =
    process.env.NODE_ENV === "production"
      ? `${process.env["NEXT_PUBLIC_API_URL"]}${path}`
      : `http://localhost:2025${path}`;
  return fetch(input, {
    credentials: "include",
    ...init,
  });
};
