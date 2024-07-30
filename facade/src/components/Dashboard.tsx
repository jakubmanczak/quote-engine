import Link from "next/link";
import { LucideMenu, LucideMessageSquareQuote } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet";
import { PropsWithChildren } from "react";
import { DesktopNavigation, MobileNavigation } from "./Navigation";
import { DashUser } from "./DashUser";

export function Dashboard(props: PropsWithChildren) {
  return (
    <div className="grid min-h-screen w-full md:grid-cols-[220px_1fr] lg:grid-cols-[280px_1fr]">
      <div className="hidden border-r bg-muted/40 md:block">
        <div className="flex h-full max-h-screen flex-col gap-2">
          <div className="flex h-14 items-center border-b px-4 lg:h-[60px] lg:px-6">
            <Link href="/" className="flex items-center gap-2 font-semibold">
              <LucideMessageSquareQuote className="h-6 w-6" />
              <span>Quote Engine</span>
            </Link>
          </div>
          <div className="flex-1">
            <DesktopNavigation />
          </div>
        </div>
      </div>
      <div className="flex flex-col">
        <header className="flex h-14 items-center gap-4 border-b bg-muted/40 px-4 lg:h-[60px] lg:px-6">
          <Sheet>
            <SheetTrigger asChild>
              <Button
                variant="outline"
                size="icon"
                className="shrink-0 md:hidden"
              >
                <LucideMenu className="h-5 w-5" />
                <span className="sr-only">Toggle navigation menu</span>
              </Button>
            </SheetTrigger>
            <SheetContent side="left" className="flex flex-col">
              <MobileNavigation />
            </SheetContent>
          </Sheet>
          <DashUser className="ml-auto" />
        </header>
        <main className="flex flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6">
          {props.children}
        </main>
      </div>
    </div>
  );
}
