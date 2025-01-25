"use client";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { qfetch } from "@/lib/utils";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default function LoginPage() {
  const [handle, setHandle] = useState("");
  const [password, setPassword] = useState("");
  const router = useRouter();

  return (
    <div className="bg-sidebar w-full h-screen flex justify-center items-center">
      <div className="bg-background rounded-lg mx-4 p-12 py-10 min-w-80 sm:min-w-96">
        <h1 className="font-fancy text-4xl text-center mb-8">
          {"Authentication"}
        </h1>
        <form method="post" className="flex flex-col">
          <label htmlFor="handle">Handle</label>
          <div className="relative">
            <span className="absolute top-[5px] left-3 text-middleground">
              {"@"}
            </span>
            <Input
              id="handle"
              placeholder="your_handle"
              className="mb-4 pl-[29px]"
              value={handle}
              onChange={(e) => setHandle(e.target.value)}
            />
          </div>
          <label htmlFor="password">Password</label>
          <Input
            id="password"
            type="password"
            className="mb-4"
            placeholder="your_password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
          <Button
            onClick={async (e) => {
              e.preventDefault();
              const res = await qfetch("/auth/login", {
                method: "POST",
                headers: {
                  "Content-Type": "application/json",
                },
                body: JSON.stringify({
                  login: handle,
                  passw: password,
                }),
              });
              if (res.ok) {
                router.push("/");
              } else {
                console.error("Login failed");
              }
            }}
          >
            Log in
          </Button>
        </form>
        <hr className="my-4" />
        <p className="mt-4 text-middleground text-sm">
          {"Login troubles? Contact your administrator."}
        </p>
      </div>
    </div>
  );
}
