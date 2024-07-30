"use client";
import {
  LucideContact,
  LucideFileClock,
  LucideLayoutDashboard,
  LucideMessageSquareQuote,
  LucideScroll,
  LucideUsers,
} from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";

const navigation: {
  name: string;
  href: string;
  icon: JSX.Element;
  disabled?: boolean;
}[] = [
  {
    name: "Dashboard",
    href: "/",
    icon: <LucideLayoutDashboard />,
  },
  {
    name: "Quotes",
    href: "#quotes",
    icon: <LucideScroll />,
    disabled: true,
  },
  {
    name: "Authors",
    href: "#quote-authors",
    icon: <LucideContact />,
    disabled: true,
  },
  {
    name: "Users",
    href: "/users",
    icon: <LucideUsers />,
  },
  {
    name: "Logs",
    href: "#logs",
    icon: <LucideFileClock />,
    disabled: true,
  },
];

const DesktopNavigation = () => {
  const pathname = usePathname();
  return (
    <nav className="grid items-start px-2 text-sm font-medium lg:px-4">
      {navigation.map((link) => {
        return (
          <Link
            href={link.href}
            key={link.href}
            className={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary --hover:bg-muted ${
              pathname === link.href
                ? "bg-muted text-primary"
                : "text-muted-foreground"
            } ${link.disabled && "opacity-[.3]"}`}
          >
            {link.icon}
            {link.name}
          </Link>
        );
      })}
    </nav>
  );
};

const MobileNavigation = () => {
  const pathname = usePathname();
  return (
    <nav className="grid gap-2 text-lg font-medium">
      <Link href="#" className="flex items-center gap-2 text-lg font-semibold">
        <LucideMessageSquareQuote className="h-6 w-6" />
        <span className="sr-only">Quote Engine</span>
      </Link>
      {navigation.map((link) => {
        return (
          <Link
            href={link.href}
            key={link.href}
            className={`mx-[-0.65rem] flex items-center gap-4 rounded-xl px-3 py-2 hover:text-foreground ${
              pathname === link.href
                ? "bg-muted text-foreground"
                : "text-muted-foreground"
            } ${link.disabled && "opacity-[.3]"}`}
          >
            {link.icon}
            {link.name}
          </Link>
        );
      })}
    </nav>
  );
};

export { DesktopNavigation, MobileNavigation };
