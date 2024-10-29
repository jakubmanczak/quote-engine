"use client";

import { Dashboard } from "@/components/Dashboard";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { qfetch } from "@/lib/qfetch";
import { user } from "@/types/user";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { toast } from "sonner";

export default function page({ params }: { params: { id: string } }) {
  const router = useRouter();

  const [user, setUser] = useState<user | null>(null);
  const [target, setTarget] = useState<user | "invalid" | "none">("none");
  const [newPass, setNewPass] = useState<string>("");

  const getSelfUser = async () => {
    const res = await qfetch("/users/self");
    if (!res.ok) return;
    const resuser = await res.json();
    setUser(resuser);
  };
  const getTargetUser = async () => {
    const res = await qfetch(`/users/${params.id}`);
    if (!res.ok) {
      setTarget("invalid");
      return;
    }
    const resuser = await res.json();
    setTarget(resuser);
  };

  const sendPasswordChangeRequest = () => {
    if (target === "invalid" || target === "none") return;
    qfetch(`/users/${target.id}/changepassword`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        pass: newPass,
      }),
    }).then((res) => {
      if (res.ok) {
        toast("Password changed!");
        if (user?.id === target.id) {
          qfetch("/auth/clear");
          router.push("/login");
        } else {
          router.push("/users");
        }
      } else {
        toast("Something went wrong...");
      }
    });
  };

  useEffect(() => {
    getSelfUser();
    getTargetUser();
  }, []);

  // prettier-ignore
  const ableToEdit: boolean =
    (!!user && target !== 'invalid' && target !== 'none') && (
      user.id === target.id || 
      user.perms.includes('Everything') ||
      (user.perms.includes('MutateUsersPasswords') && target.perms.includes('Everything'))
    );

  return (
    <Dashboard>
      {!user && (
        <h1 className="text-center">
          {"You must be logged in to access this."}
        </h1>
      )}
      {target === "invalid" && !!user && (
        <>
          <h1 className="text-center">{"No results for user id."}</h1>
        </>
      )}
      {ableToEdit && (
        <div className="flex flex-col items-center justify-center gap-16">
          <div>
            <h1 className="text-center font-semibold text-2xl">
              {"Password Change"}
            </h1>
            <p className="text-center">
              <span className="text-sm text-neutral-500">({user?.id})</span>{" "}
              <br />
              {"For user:"} {user?.name}
            </p>
          </div>
          <div className="max-w-sm w-full">
            <form>
              <label htmlFor="newpass">{"New password"}</label>
              <Input
                id="newpass"
                type="password"
                autoComplete="new-password"
                className="mb-8"
                value={newPass}
                onChange={(e) => {
                  setNewPass(e.target.value);
                }}
              />
              <Button
                className="w-full"
                onClick={(e) => {
                  e.preventDefault();
                  sendPasswordChangeRequest();
                }}
              >
                {"Submit"}
              </Button>
            </form>
          </div>
        </div>
      )}
    </Dashboard>
  );
}
