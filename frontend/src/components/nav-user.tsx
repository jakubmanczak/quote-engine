"use client";

import { BadgeCheck, ChevronsUpDown, LogOut, LucideUser } from "lucide-react";

import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from "@/components/ui/sidebar";
import { useEffect, useState } from "react";
import { Button } from "./ui/button";
import Link from "next/link";
import { qfetch } from "@/lib/utils";

export function NavUser() {
  const { isMobile } = useSidebar();
  const [user, setUser] = useState<User | "none">("none");

  const fetchUser = async () => {
    const res = await qfetch("/users/me");
    if (res.ok) {
      const data = await res.json();
      setUser(data);
    } else {
      setUser("none");
    }
  };
  const logout = async () => {
    const res = await qfetch("/auth/clear", { method: "POST" });
    if (res.ok) {
      setUser("none");
    }
  };

  useEffect(() => {
    fetchUser();
  }, []);

  if (user === "none") {
    return (
      <Link href={"/auth"} className="">
        <Button variant={"secondary"} className="w-full h-12">
          Log in
        </Button>
      </Link>
    );
  }

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton
              size="lg"
              className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
            >
              <Avatar className="h-8 w-8 rounded-lg">
                <AvatarImage src={user.avatar} alt={"You"} />
                <AvatarFallback className="rounded-lg">
                  <LucideUser className="size-4" />
                </AvatarFallback>
              </Avatar>
              <div className="grid flex-1 text-left text-sm leading-tight">
                <span className="truncate font-semibold">{"You"}</span>
                <span className="truncate text-xs">{`@${user.handle}`}</span>
              </div>
              <ChevronsUpDown className="ml-auto size-4" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-[--radix-dropdown-menu-trigger-width] min-w-56 rounded-lg"
            side={isMobile ? "bottom" : "right"}
            align="end"
            sideOffset={4}
          >
            <DropdownMenuLabel className="p-0 font-normal">
              <div className="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                <Avatar className="h-8 w-8 rounded-lg">
                  <AvatarImage src={user.avatar} alt={"You"} />
                  <AvatarFallback className="rounded-lg">
                    <LucideUser className="size-4" />
                  </AvatarFallback>
                </Avatar>
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-semibold">{"You"}</span>
                  <span className="truncate text-xs">{user.handle}</span>
                </div>
              </div>
            </DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuGroup>
              {/* <DropdownMenuItem>
                <BadgeCheck />
                Account
              </DropdownMenuItem> */}
              <DropdownMenuItem
                className="cursor-pointer"
                onClick={() => logout()}
              >
                <LogOut />
                Log out
              </DropdownMenuItem>
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  );
}
