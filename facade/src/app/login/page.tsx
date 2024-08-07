"use client";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { toast } from "sonner";

export default function LoginPage() {
  const router = useRouter();
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const sendloginrequest = async () => {
    const res = await fetch("http://localhost:2019/auth/login", {
      method: "POST",
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify({
        username,
        password,
      }),
      credentials: "include",
    });
    if (res.ok) {
      router.push("/");
      toast("Logged in sucessfully.");
    } else {
      const body = await res.text();
      toast(body);
    }
  };
  return (
    <div className="flex flex-col min-h-screen p-4">
      <div className="flex-1 flex flex-col items-center w-full h-full rounded-md bg-muted p-4">
        <h2 className="text-4xl font-medium mt-16">Quote Engine</h2>
        <Card className="mx-auto max-w-sm w-full mt-12">
          <CardHeader>
            <CardTitle className="text-2xl">Login</CardTitle>
            <CardDescription>
              Enter your credentials to get started.
            </CardDescription>
          </CardHeader>
          <CardContent className="grid gap-4">
            <div className="grid gap-2">
              <Label id="usernamelabel" htmlFor="username">
                Username
              </Label>
              <Input
                id="username"
                required
                value={username}
                onChange={(e) => setUsername(e.target.value)}
              />
            </div>
            <div className="grid gap-2">
              <Label id="passwordlabel" htmlFor="password">
                Password
              </Label>
              <Input
                id="password"
                type="password"
                required
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") sendloginrequest();
                }}
              />
            </div>
          </CardContent>
          <CardFooter>
            <Button className="w-full" onClick={() => sendloginrequest()}>
              Sign in
            </Button>
          </CardFooter>
        </Card>
        <p className="mt-auto text-muted-foreground">
          Quote Engine uses cookies to handle authentication.
        </p>
      </div>
    </div>
  );
}
