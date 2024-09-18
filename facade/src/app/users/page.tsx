"use client";
import { CreateUser } from "@/components/CreateUser";
import { Dashboard } from "@/components/Dashboard";
import { DialogDrawer } from "@/components/DialogDrawer";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { qfetch } from "@/lib/qfetch";
import { user } from "@/types/user";
import {
  LucideFlower,
  LucidePaintbrush,
  LucideShieldCheck,
  LucideUser,
  LucideWrench,
} from "lucide-react";
import { useEffect, useState } from "react";

export default function UsersPage() {
  const [user, setUser] = useState<user | null>(null);
  const [users, setUsers] = useState<user[]>([]);

  // all users, but current logged in is in front
  const userslist = [user]
    .concat(
      users.filter((u) => {
        return u.id !== user?.id;
      })
    )
    .filter((el) => {
      return el !== null;
    });

  const [fetchStat, setFetchStat] = useState<{
    status: number;
    statusText: string;
  } | null>(null);

  const getUsers = async () => {
    const res = await qfetch("/users");
    setFetchStat({
      status: res.status,
      statusText: res.statusText,
    });
    if (!res.ok) {
      setUsers([]);
      return;
    }

    const array: user[] = await res.json();
    setUsers(array);
  };

  const getUser = async () => {
    const res = await qfetch("/users/self");
    if (!res.ok) return;
    const resuser = await res.json();
    setUser(resuser);
  };

  useEffect(() => {
    getUsers();
    getUser();
  }, []);

  return (
    <Dashboard>
      {fetchStat?.status === 200 && (
        <div className="flex flex-row gap-4 items-center">
          <p className="text-xl">Users</p>
          {user?.perms.includes("CreateUsers") ||
            (user?.perms.includes("Everything") && (
              <DialogDrawer
                buttonText="Add new user"
                contentTitle="Adding a new user"
                contentDescr="Input their username and starting password here."
              >
                <CreateUser />
              </DialogDrawer>
            ))}
        </div>
      )}
      {fetchStat?.status === 200 && (
        <div className="flex flex-row flex-wrap gap-4 justify-center sm:justify-normal">
          {userslist.map((u) => {
            return (
              <Card
                key={u.id}
                className="flex flex-row items-center relative flex-1 min-w-64 max-w-80 overflow-hidden"
              >
                {u.picture.length ? (
                  <img
                    src={u.picture}
                    alt={`${u.name}'s photo`}
                    className="h-24 w-24"
                  />
                ) : (
                  <div
                    className="w-24 h-24"
                    style={{ backgroundColor: `#${u.color}` }}
                  >
                    <LucideUser className="mx-auto my-auto h-full scale-[2.25] text-white mix-blend-exclusion" />
                  </div>
                )}
                <div className="flex flex-col justify-between p-3 pb-2 h-full self-start flex-1">
                  <div className="flex flex-row items-center w-full text-muted-foreground gap-1">
                    <h3 className="font-semibold text-xl mr-auto text-black">
                      {u.name}
                    </h3>
                    {u.perms.includes("Everything") && <LucideShieldCheck />}
                    {u.perms.includes("DisplayFlower") && <LucideFlower />}
                    <div
                      className="block w-5 h-5 rounded-full"
                      style={{
                        backgroundColor: `#${u.color}`,
                      }}
                    />
                  </div>
                  <div className="flex flex-row justify-end items-center w-full gap-2">
                    {user?.perms.includes("Everything") && (
                      <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                          <Button variant="outline" className="rounded-full">
                            <LucideWrench />
                          </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent>
                          <p className="p-1">Edit administrative attributes.</p>
                        </DropdownMenuContent>
                      </DropdownMenu>
                    )}
                    {u.id === user?.id && (
                      <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                          <Button
                            variant="default"
                            size={"icon"}
                            className="rounded-full"
                          >
                            <LucidePaintbrush className="scale-[.9]" />
                          </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent>
                          <p className="p-1">Edit your cosmetic attributes.</p>
                        </DropdownMenuContent>
                      </DropdownMenu>
                    )}
                  </div>
                </div>
              </Card>
            );
          })}
        </div>
      )}
      {fetchStat !== null && fetchStat.status !== 200 && (
        <>
          <h1 className="text-2xl sm:text-4xl text-center">{`${fetchStat.status} - ${fetchStat.statusText}`}</h1>
        </>
      )}
    </Dashboard>
  );
}
