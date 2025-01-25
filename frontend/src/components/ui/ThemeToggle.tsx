"use client";

import * as React from "react";
import { MoonIcon, SunIcon, SunMoonIcon } from "lucide-react";

import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";
import { useTheme } from "next-themes";

export function ThemeToggle() {
  const [mounted, setMounted] = useState(false);
  const { theme, setTheme, resolvedTheme } = useTheme();

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) {
    return (
      <Button variant={"ghost"} size={"icon"} className="h-7 w-7">
        <div className="bg-primary opacity-[.2] h-4 w-4 rounded animate-pulse" />
        <div className="sr-only">Toggle theme</div>
      </Button>
    );
  }

  return (
    <Button
      variant="ghost"
      size="icon"
      onClick={() =>
        setTheme(
          // switch user between their system theme and
          // the theme opposite their system resolved theme
          theme === "system" && resolvedTheme === "dark"
            ? "light"
            : theme === "system" && resolvedTheme === "light"
              ? "dark"
              : "system",
        )
      }
      className="h-7 w-7"
    >
      {theme === "dark" && <MoonIcon className="h-4 w-4" />}
      {theme === "light" && <SunIcon className="h-4 w-4" />}
      {theme === "system" && <SunMoonIcon className="h-4 w-4" />}
      <span className="sr-only">Toggle theme</span>
    </Button>
  );
}
