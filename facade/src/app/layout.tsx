import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { Dashboard } from "../components/Dashboard";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Quote Engine",
  description: "",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${inter.className} min-h-screen`}>
        <Dashboard>{children}</Dashboard>
      </body>
    </html>
  );
}