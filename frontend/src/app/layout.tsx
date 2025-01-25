import type { Metadata } from "next";
import { Markazi_Text, Montserrat } from "next/font/google";
import "./globals.css";
// import { GrainEffect } from "@/components/GrainEffect";
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
        <ThemeProvider attribute={["class"]}>{children}</ThemeProvider>
      </body>
    </html>
  );
}
