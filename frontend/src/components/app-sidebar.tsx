"use client";

import * as React from "react";
import {
  LucideActivity,
  LucideContact,
  LucideFileClock,
  LucideGithub,
  LucideLayoutDashboard,
  LucideScroll,
  LucideUsers,
} from "lucide-react";

import { NavMain } from "@/components/nav-main";
import { NavSecondary } from "@/components/nav-secondary";
import { NavUser } from "@/components/nav-user";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import Link from "next/link";

const data = {
  navMain: [
    {
      title: "Dashboard",
      url: "/dashboard",
      icon: LucideLayoutDashboard,
    },
    {
      title: "Quotes",
      url: "/",
      icon: LucideScroll,
      isActive: true,
      items: [
        {
          title: "Search",
          url: "/",
        },
        {
          title: "Recent",
          disabled: true,
          url: "/",
        },
        {
          title: "Create",
          url: "/",
        },
      ],
    },
    {
      title: "Authors",
      url: "/authors",
      icon: LucideContact,
    },
    {
      title: "Users",
      url: "/",
      icon: LucideUsers,
    },
    {
      title: "Logs",
      url: "/",
      icon: LucideFileClock,
    },
    {
      title: "System Health",
      url: "/",
      icon: LucideActivity,
    },
  ],
  navSecondary: [
    {
      title: "Source Code",
      url: "https://github.com/jakubmanczak/quote-engine",
      icon: LucideGithub,
    },
  ],
};

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar variant="inset" {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" asChild>
              <Link href="/">
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-semibold font-fancy text-3xl">
                    Quote Engine
                  </span>
                  {/* <span className="truncate text-xs">
                    {"Powered by Rust/React"}
                  </span> */}
                </div>
              </Link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <NavMain items={data.navMain} />
        <NavSecondary items={data.navSecondary} className="mt-auto" />
      </SidebarContent>
      <SidebarFooter>
        <NavUser />
      </SidebarFooter>
    </Sidebar>
  );
}
