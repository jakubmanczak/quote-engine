"use client";

import { qfetch } from "@/lib/utils";
import { DropdownMenuItem } from "../ui/dropdown-menu";

export const DeleteAuthorButton = (props: { id: string }) => {
  const fetch = async () => {
    const res = await qfetch(`/authors/${props.id}`, {
      method: "DELETE",
      credentials: "include",
    });
    if (res.ok) {
      location.reload();
    }
  };
  return (
    <DropdownMenuItem
      className="cursor-pointer text-red-600"
      onClick={() => fetch()}
    >
      {"Delete"}
    </DropdownMenuItem>
  );
};
