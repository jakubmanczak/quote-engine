"use client";
import { useState } from "react";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { qfetch } from "@/lib/qfetch";
import { toast } from "sonner";
import { useRouter } from "next/navigation";

const CreateUser = (props: { userRefresh: () => void }) => {
  const [user, setUser] = useState<string>("");
  const [pass, setPass] = useState<string>("");
  return (
    <div className="flex flex-col gap-1 py-4">
      <p>Username</p>
      <Input
        className="mb-4"
        type="text"
        value={user}
        onChange={(e) => setUser(e.target.value)}
      />
      <p>Password</p>
      <Input
        className="mb-4"
        type="password"
        value={pass}
        onChange={(e) => setPass(e.target.value)}
      />
      <Button
        onClick={() => {
          qfetch("/users", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              name: user,
              pass,
            }),
          }).then((res) => {
            if (res.ok) {
              toast("User succesfully created!");
            } else {
              toast("Something went wrong...");
            }
            props.userRefresh();
          });
        }}
      >
        {"Create user"}
      </Button>
    </div>
  );
};

export { CreateUser };
