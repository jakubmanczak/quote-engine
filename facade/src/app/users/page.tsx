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
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { toast } from "sonner";

export default function UsersPage() {
  const router = useRouter();
  const [user, setUser] = useState<user | null>(null);
  const [users, setUsers] = useState<user[]>([]);

  const [dwiOpen, setDwiOpen] = useState<boolean>(false);
  const [dwiAction, setDwiAction] = useState<"name" | "pic" | "clr" | "delete">(
    "name"
  );

  const [editUserId, setEditUserId] = useState<string>("");
  const [editUsername, setEditUsername] = useState<string>("");
  const [editColor, setEditColor] = useState<string>("");

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

  const submitEditUsername = () => {
    qfetch(`/users/${editUserId}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name: editUsername,
      }),
    }).then((res) => {
      if (res.ok) {
        toast("Username changed successfully!");
      } else {
        toast("Something went wrong...");
      }
      getUsers();
      getUser();
      setDwiOpen(false);
    });
  };

  const submitEditColour = () => {
    qfetch(`/users/${editUserId}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        color: editColor,
      }),
    }).then((res) => {
      if (res.ok) {
        toast("Colour changed successfully!");
      } else {
        toast("Something went wrong...");
      }
      getUsers();
      getUser();
      setDwiOpen(false);
    });
  };

  const submitDeleteUser = () => {
    qfetch(`/users/${editUserId}`, {
      method: "DELETE",
    }).then((res) => {
      if (res.ok) {
        toast("User deleted.");
      } else {
        toast("Something went wrong...");
      }
      getUsers();
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
          dwiAction === "name"
            ? "Edit Username"
            : dwiAction === "pic"
            ? "Edit picture"
            : dwiAction === "clr"
            ? "Edit colour"
            : dwiAction === "delete"
            ? "User deletion"
            : "Unknown action"
        }
      >
        <div className="flex flex-col gap-1 py-4">
          {dwiAction === "name" && (
            <>
              <p>Username</p>
              <Input
                className="mb-4"
                type="text"
                autoComplete="off"
                value={editUsername}
                onChange={(e) => setEditUsername(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") submitEditUsername();
                }}
              />
            </>
          )}
          {dwiAction === "clr" && (
            <>
              <p>Colour: {editColor}</p>
              <Input
                className="mb-4"
                type="color"
                value={`#${editColor}`}
                onChange={(e) =>
                  setEditColor(e.target.value.replaceAll("#", ""))
                }
              />
            </>
          )}
          {dwiAction === "delete" && (
            <>
              <p className="text-center mb-4">
                Are you sure about this? <br />
                This action is not reversible.
              </p>
            </>
          )}
          <Button
            onClick={() => {
              switch (dwiAction) {
                case "name":
                  submitEditUsername();
                  break;
                case "clr":
                  submitEditColour();
                  break;
                case "delete":
                  submitDeleteUser();
                  break;
              }
            }}
            variant={dwiAction === "delete" ? "destructive" : "default"}
          >
            {dwiAction === "delete" ? "Yes, really" : "Submit"}
          </Button>
        </div>
      </DialogDrawer>
      <div className="flex flex-row gap-4 items-center">
        <p className="text-xl">Users</p>
        <CreateUser
          userRefresh={getUsers}
          disabled={
            !(
              user?.perms.includes("Everything") ||
              user?.perms.includes("CreateUsers")
            )
          }
        />
      </div>
      {fetchStat?.status === 200 && (
        <div className="flex flex-row flex-wrap gap-4">
          {userslist.map((u) => {
            const editpasswordvisible =
              user?.perms.includes("Everything") ||
              (user?.perms.includes("MutateUsersPasswords") &&
                !u.perms.includes("Everything")) ||
              (user?.perms.includes("MutateOwnUser") && user.id === u.id);
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
                                setDwiAction("name");
                                setEditUserId(u.id);
                                setEditUsername(u.name);
                                setDwiOpen(true);
                              }}
                            >
                              Edit username
                            </DropdownMenuItem>
                            <DropdownMenuItem
                              disabled
                              className="cursor-pointer"
                            >
                              Edit picture
                            </DropdownMenuItem>
                            <DropdownMenuItem
                              className="cursor-pointer"
                              onClick={() => {
                                setDwiAction("clr");
                                setEditUserId(u.id);
                                setEditColor(u.color);
                                setDwiOpen(true);
                              }}
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
                        {editpasswordvisible && (
                          <Link href={`/users/${u.id}/change-password`}>
                            <DropdownMenuItem className="cursor-pointer">
                              {"New password"}
                            </DropdownMenuItem>
                          </Link>
                        )}
                        {(user?.perms.includes("Everything") ||
                          user?.perms.includes("DeleteUsers")) && (
                          <DropdownMenuItem
                            disabled={u.id === user.id}
                            className="cursor-pointer text-red-600"
                            onClick={() => {
                              if (u.id === user.id) return;
                              setDwiAction("delete");
                              setEditUserId(u.id);
                              setDwiOpen(true);
                            }}
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
          <h1>{"You must be logged in to access this."}</h1>
        </>
      )}
    </Dashboard>
  );
}
