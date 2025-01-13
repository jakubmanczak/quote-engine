import type { Metadata } from "next";
import { Markazi_Text, Montserrat } from "next/font/google";
import "./globals.css";
import { GrainEffect } from "@/components/GrainEffect";
import { Navigation } from "@/components/Navigation";

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
    <html lang="en">
      <body
        className={`${markazi.variable} ${montserrat.className} antialiased min-h-screen w-full relative`}
      >
        <GrainEffect />
        <div className="relative z-10 min-h-screen w-full flex flex-col justify-normal items-start">
          <Navigation />
          {children}
        </div>
      </body>
    </html>
  );
}
