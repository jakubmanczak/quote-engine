"use client";
import { DropdownMenuItem } from "../ui/dropdown-menu";

export const CopyAuthorID = (props: { id: string }) => {
  return (
    <DropdownMenuItem
      className="cursor-pointer"
      onClick={async () => {
        await navigator.clipboard.writeText(props.id);
      }}
    >
      {"Copy Author ID"}
    </DropdownMenuItem>
  );
};
