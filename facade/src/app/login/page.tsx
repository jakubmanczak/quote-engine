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
import { useState } from "react";
import { toast } from "sonner";

export default function LoginPage() {
  // fetch("http://localhost:2019/cookie", {
  //   credentials: "include",
  // });
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const sendloginrequest = () => {
    toast("Login functionality unimplemented!");
  };
  return (
    <div className="flex flex-col items-center w-full h-full rounded-md bg-muted p-4">
      <Card className="mx-auto mt-32 max-w-sm w-full">
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
    </div>
  );
}
