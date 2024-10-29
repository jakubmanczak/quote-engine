"use client";

import { Dashboard } from "@/components/Dashboard";
import { DialogDrawer } from "@/components/DialogDrawer";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuPortal,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { qfetch } from "@/lib/qfetch";
import { user } from "@/types/user";
import { LucideScrollText, LucideWrench } from "lucide-react";
import { useEffect, useState } from "react";
import { toast } from "sonner";

type author = {
  id: string;
  name: string;
  obfname: string;
};

export default function Page() {
  const [user, setUser] = useState<user | null>(null);
  const [authors, setAuthors] = useState<author[] | null>(null);

  const [dwiOpen, setDwiOpen] = useState<boolean>(false);
  const [newName, setNewName] = useState<string>("");
  const [newObf, setNewObf] = useState<string>("");

  const getUser = async () => {
    const res = await qfetch("/users/self");
    if (!res.ok) return;
    const resuser = await res.json();
    setUser(resuser);
  };

  const getAuthors = async () => {
    const res = await qfetch("/authors");
    if (!res.ok) return;
    const authors = await res.json();
    console.log(authors);
    setAuthors(authors);
  };

  const sendNewAuthor = async () => {
    qfetch("/authors", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name: newName,
        obfname: newObf,
      }),
    }).then((res) => {
      if (res.ok) {
        toast("Author added successfully!");
      } else {
        toast("Something went wrong...");
      }
      getAuthors();
      setNewName("");
      setNewObf("");
      setDwiOpen(false);
    });
  };

  useEffect(() => {
    getUser();
    getAuthors();
  }, []);

  return (
    <Dashboard>
      <div className="flex items-center gap-4">
        <h1 className="text-xl">Authors</h1>
        <Button variant={"outline"} onClick={() => setDwiOpen(true)}>
          Add new author
        </Button>
        <DialogDrawer
          open={dwiOpen}
          setOpen={setDwiOpen}
          contentTitle="Add new author"
        >
          <div className="flex flex-col gap-1 py-4">
            <Label htmlFor="newname">Author name</Label>
            <Input
              id="newname"
              type="text"
              value={newName}
              onChange={(e) => {
                setNewName(e.target.value);
              }}
              className="mb-4"
            />
            <Label htmlFor="obfname">Obfuscated name</Label>
            <Input
              id="obfname"
              type="text"
              value={newObf}
              onChange={(e) => {
                setNewObf(e.target.value);
              }}
              className="mb-4"
            />
            <Button onClick={() => sendNewAuthor()}>{"Submit"}</Button>
          </div>
        </DialogDrawer>
      </div>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        {!!authors &&
          authors.map((a) => {
            return (
              <span key={a.id}>
                <Author data={a} />
              </span>
            );
          })}
      </div>
    </Dashboard>
  );
}

const Author = ({ data, ...props }: { data: author }) => {
  return (
    <Card className="p-4 flex flex-col items-center gap-4">
      <div className="text-center">
        <h3 className="sm:text-lg font-medium">{data.name}</h3>
        <p className="text-sm text-neutral-400">{data.obfname}</p>
      </div>
      <hr className="w-full mt-auto" />
      <div className="flex gap-4 w-full">
        <div className="text-center w-full">
          <p className="uppercase font-semibold text-neutral-600">{"quotes"}</p>
          <h3 className="text-3xl">{"17"}</h3>
        </div>
        <div className="text-center w-full">
          <p className="uppercase font-semibold text-neutral-600">{"lines"}</p>
          <h3 className="text-3xl">{"33"}</h3>
        </div>
      </div>
      <div className="pt-2 flex flex-row gap-2 w-full">
        <Button variant={"outline"} className="w-full flex gap-2" disabled>
          <LucideScrollText />
          {"Quotes"}
        </Button>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant={"outline"} className="w-full">
              <LucideWrench />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent>
            <DropdownMenuSub>
              <DropdownMenuSubTrigger>{"Names"}</DropdownMenuSubTrigger>
              <DropdownMenuPortal>
                <DropdownMenuSubContent>
                  <DropdownMenuItem className="cursor-pointer" disabled>
                    Modify name
                  </DropdownMenuItem>
                  <DropdownMenuItem className="cursor-pointer" disabled>
                    Modify codename
                  </DropdownMenuItem>
                </DropdownMenuSubContent>
              </DropdownMenuPortal>
            </DropdownMenuSub>
            <DropdownMenuItem
              className="cursor-pointer"
              onClick={() => {
                navigator.clipboard.writeText(data.id);
                toast(`Author ID copied!`);
              }}
            >
              {"Copy author ID"}
            </DropdownMenuItem>
            <DropdownMenuItem className="cursor-pointer text-red-600" disabled>
              {"Delete author"}
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </Card>
    // <Card className="p-2 px-4 flex items-center gap-2">
    //   <div>
    //     <h3 className="sm:text-lg">{data.name}</h3>
    //     <p className="text-sm">
    //       <span className="text-neutral-300">{"("}</span>
    //       <span className="text-neutral-400">{data.obfname}</span>
    //       <span className="text-neutral-300">{")"}</span>
    //     </p>
    //   </div>
    //   <Button className="ml-auto" disabled variant={"outline"}>
    //     <LucideWrench />
    //   </Button>
    //   <Button className="" disabled variant={"outline"}>
    //     <LucideTrash2 />
    //   </Button>
    // </Card>
  );
};
