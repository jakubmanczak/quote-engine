"use client";
import { CreateUser } from "@/components/CreateUser";
import { Dashboard } from "@/components/Dashboard";
import { DialogDrawer } from "@/components/DialogDrawer";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { qfetch } from "@/lib/qfetch";
import { user } from "@/types/user";
import {
  LucideFlower,
  LucideShieldCheck,
  LucideUser,
  LucideWrench,
} from "lucide-react";
import { useEffect, useState } from "react";
import { toast } from "sonner";

export default function UsersPage() {
  const [user, setUser] = useState<user | null>(null);
  const [users, setUsers] = useState<user[]>([]);

  const [dwiOpen, setDwiOpen] = useState<boolean>(false);
  const [dwiAction, setDwiAction] = useState<"nick" | "pic" | "clr">("nick");

  const [editUserId, setEditUserId] = useState<string>("");
  const [editNickname, setEditNickname] = useState<string>("");

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

  const submitEditNickname = () => {
    qfetch(`/users/${editUserId}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name: editNickname,
      }),
    }).then((res) => {
      if (res.ok) {
        toast("Nickname changed successfully!");
      } else {
        toast("Something went wrong...");
      }
      getUsers();
      getUser();
      setDwiOpen(false);
    });
  };

  useEffect(() => {
    getUsers();
    getUser();
  }, []);

  return (
    <Dashboard>
      <DialogDrawer
        open={dwiOpen}
        setOpen={setDwiOpen}
        contentTitle={
          dwiAction === "nick"
            ? "Edit nickname"
            : dwiAction === "pic"
            ? "Edit picture"
            : dwiAction === "clr"
            ? "Edit colour"
            : "Unknown action"
        }
      >
        <div className="flex flex-col gap-1 py-4">
          {dwiAction === "nick" && (
            <>
              <p>Nickname</p>
              <Input
                className="mb-4"
                type="text"
                value={editNickname}
                onChange={(e) => setEditNickname(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") submitEditNickname();
                }}
              />
            </>
          )}
          <Button
            onClick={() => {
              switch (dwiAction) {
                case "nick":
                  submitEditNickname();
              }
            }}
          >
            {"Submit"}
          </Button>
        </div>
      </DialogDrawer>
      {fetchStat?.status === 200 && (
        <div className="flex flex-row gap-4 items-center">
          <p className="text-xl">Users</p>
          {user?.perms.includes("CreateUsers") ||
            (user?.perms.includes("Everything") && (
              <CreateUser userRefresh={getUsers} />
            ))}
        </div>
      )}
      {fetchStat?.status === 200 && (
        <div className="flex flex-row flex-wrap gap-4">
          {userslist.map((u) => {
            return (
              <Card
                key={u.id}
                className="flex flex-row items-center relative flex-1 min-w-80 max-w-80 overflow-hidden"
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
                    <DropdownMenu>
                      <DropdownMenuTrigger asChild>
                        <Button
                          variant={u.id === user?.id ? "default" : "outline"}
                          size={u.id === user?.id ? "default" : "icon"}
                          className="rounded-full"
                        >
                          {u.id === user?.id && (
                            <span className="mr-1">{"Settings"}</span>
                          )}
                          <LucideWrench className="scale-[.9]" />
                        </Button>
                      </DropdownMenuTrigger>
                      <DropdownMenuContent>
                        {(user?.perms.includes("Everything") ||
                          user?.perms.includes("MutateUsers") ||
                          user?.id === u.id) && (
                          <>
                            <DropdownMenuItem
                              className="cursor-pointer"
                              onClick={() => {
                                setDwiAction("nick");
                                setEditUserId(u.id);
                                setEditNickname(u.name);
                                setDwiOpen(true);
                              }}
                            >
                              Edit nickname
                            </DropdownMenuItem>
                            <DropdownMenuItem
                              disabled
                              className="cursor-pointer"
                            >
                              Edit picture
                            </DropdownMenuItem>
                            <DropdownMenuItem
                              disabled
                              className="cursor-pointer"
                            >
                              Edit colour
                            </DropdownMenuItem>
                            <DropdownMenuSeparator />
                          </>
                        )}
                        <DropdownMenuItem
                          className="cursor-pointer"
                          onClick={() => {
                            navigator.clipboard.writeText(u.id);
                            toast(`User ID copied!`);
                          }}
                        >
                          {"Copy user ID"}
                        </DropdownMenuItem>
                        {(user?.perms.includes("Everything") ||
                          user?.perms.includes("MutateUsersPermissions")) && (
                          <DropdownMenuItem disabled className="cursor-pointer">
                            Permissions
                          </DropdownMenuItem>
                        )}
                        {(user?.perms.includes("Everything") ||
                          user?.perms.includes("DeleteUsers")) && (
                          <DropdownMenuItem
                            disabled
                            className="cursor-pointer text-red-600"
                          >
                            Delete user
                          </DropdownMenuItem>
                        )}
                      </DropdownMenuContent>
                    </DropdownMenu>
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
