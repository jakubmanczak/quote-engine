"use client";
import { LucideCircleUser } from "lucide-react";
import { Button } from "./ui/button";
import Link from "next/link";
import { useEffect, useState } from "react";
import { user } from "@/types/user";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "./ui/dropdown-menu";
import { useRouter } from "next/navigation";

const DashUser = (props: { className?: string }) => {
  const router = useRouter();
  const [user, setUser] = useState<user | "loggedout">("loggedout");
  const getUserState = async () => {
    const res = await fetch("http://localhost:2019/users/self", {
      credentials: "include",
    });
    if (!res.ok) {
      setUser("loggedout");
      return;
    }

    const json = await res.json();
    const assembleduser: user = {
      id: json["id"],
      name: json["name"],
      color: json["color"],
      picture: json["picture"],
      perms: json["perms"],
    };
    setUser(assembleduser);
  };
  const logOut = async () => {
    const res = await fetch("http://localhost:2019/auth/clear", {
      credentials: "include",
    });
    setUser("loggedout");
    router.push("/");
  };
  useEffect(() => {
    getUserState();
  }, []);
  return (
    <>
      {user === "loggedout" && (
        <Link href={"/login"} className={props.className}>
          <Button variant={"outline"} className="flex gap-2">
            <LucideCircleUser className="h-5 w-5" />
            Log in
          </Button>
        </Link>
      )}
      {user !== "loggedout" && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild className={props.className}>
            <Button variant="outline">
              <LucideCircleUser className="h-5 w-5 mr-1" />
              <p>{user.name}</p>
              <span className="sr-only">Toggle user menu</span>
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuLabel>My Account</DropdownMenuLabel>
            {/* <DropdownMenuSeparator />
            <DropdownMenuItem>Settings</DropdownMenuItem>
            <DropdownMenuItem>Support</DropdownMenuItem> */}
            <DropdownMenuSeparator />
            <DropdownMenuItem
              className="cursor-pointer"
              onClick={() => logOut()}
            >
              Logout
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )}
    </>
  );
};

export { DashUser };
