import type { Metadata } from "next";
import { Markazi_Text, Montserrat } from "next/font/google";
import "./globals.css";
// import { GrainEffect } from "@/components/GrainEffect";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import { ThemeToggle } from "@/components/ui/ThemeToggle";
import { ThemeProvider } from "next-themes";

const markazi = Markazi_Text({
  variable: "--font-markazi",
  subsets: ["latin"],
});

const montserrat = Montserrat({
  variable: "--font-montserrat",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Quote Engine",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body
        className={`${markazi.variable} ${montserrat.className} antialiased min-h-screen w-full relative`}
      >
        <ThemeProvider attribute={["class"]}>
          <SidebarProvider>
            <AppSidebar />
            <SidebarInset>
              <header className="flex h-16 shrink-0 items-center gap-2">
                <div className="flex items-center gap-2 px-4">
                  {/* <SidebarTrigger className="-ml-1" /> */}
                  <ThemeToggle />
                </div>
              </header>
              <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
                {children}
              </div>
            </SidebarInset>
          </SidebarProvider>
        </ThemeProvider>
        {/* <GrainEffect /> */}
        {/* <div className="relative z-10 min-h-screen w-full flex flex-col justify-normal items-start"> */}
        {/* <Navigation /> */}
        {/* </div> */}
      </body>
    </html>
  );
}
