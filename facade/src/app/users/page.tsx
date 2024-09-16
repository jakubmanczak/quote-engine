"use client";
import { CreateUser } from "@/components/CreateUser";
import { Dashboard } from "@/components/Dashboard";
import { DialogDrawer } from "@/components/DialogDrawer";
import { Card } from "@/components/ui/card";
import { qfetch } from "@/lib/qfetch";
import { user } from "@/types/user";
import { LucideShieldCheck, LucideUser } from "lucide-react";
import { useEffect, useState } from "react";

export default function UsersPage() {
  const [users, setUsers] = useState<user[]>([]);
  const [user, setUser] = useState<user | null>(null);
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
          {users.map((user) => {
            return (
              <Card
                key={user.id}
                className="flex flex-col p-4 flex-1 min-w-64 max-w-80"
              >
                <div className="flex flex-row gap-4 items-center relative">
                  {user.picture.length ? (
                    <img
                      src={user.picture}
                      alt={`${user.name}'s photo`}
                      className="rounded-full h-16 w-16"
                    />
                  ) : (
                    <div className="w-16 h-16 bg-slate-500 rounded-full">
                      <LucideUser className="mx-auto mt-5 scale-[1.5] text-white" />
                    </div>
                  )}
                  <h3 className="font-semibold text-xl text-center">
                    {user.name}
                  </h3>
                  <div className="absolute top-0 right-0 flex flex-row items-center gap-1 text-muted-foreground">
                    {user.perms.includes("Everything") && <LucideShieldCheck />}
                    <div
                      className="block w-5 h-5 rounded-full"
                      style={{
                        backgroundColor: `#${user.color}`,
                      }}
                    />
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
