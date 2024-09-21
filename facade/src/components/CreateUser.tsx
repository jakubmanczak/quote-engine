"use client";
import { useState } from "react";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { qfetch } from "@/lib/qfetch";
import { toast } from "sonner";
import { DialogDrawer } from "./DialogDrawer";

const CreateUser = (props: { userRefresh: () => void }) => {
  const [open, setOpen] = useState<boolean>(false);
  const [user, setUser] = useState<string>("");
  const [pass, setPass] = useState<string>("");

  const submit = () => {
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
      setOpen(false);
    });
  };

  return (
    <DialogDrawer
      contentTitle="Adding a new user"
      contentDescr="Input their username and starting password here."
      trigger={<Button variant={"outline"}>{"Add new user"}</Button>}
      open={open}
      setOpen={setOpen}
    >
      <div className="flex flex-col gap-1 py-4">
        <p>Username</p>
        <Input
          className="mb-4"
          type="text"
          value={user}
          autoComplete="off"
          onChange={(e) => setUser(e.target.value)}
        />
        <p>Password</p>
        <Input
          className="mb-4"
          type="password"
          value={pass}
          autoComplete="off"
          onChange={(e) => setPass(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") submit();
          }}
        />
        <Button onClick={() => submit()}>{"Create user"}</Button>
      </div>
    </DialogDrawer>
  );
};

export { CreateUser };
