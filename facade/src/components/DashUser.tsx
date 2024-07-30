import { LucideCircleUser } from "lucide-react";
import { Button } from "./ui/button";
import Link from "next/link";

const DashUser = (props: { className?: string }) => {
  return (
    <Link href={"/login"} className={props.className}>
      <Button variant={"outline"} className="flex gap-2">
        <LucideCircleUser className="h-5 w-5" />
        Log in
      </Button>
    </Link>
  );
  {
    /*
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="secondary"
          size="icon"
          className="rounded-full ml-auto"
        >
          <LucideCircleUser className="h-5 w-5" />
          <span className="sr-only">Toggle user menu</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuLabel>My Account</DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuItem>Settings</DropdownMenuItem>
        <DropdownMenuItem>Support</DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem>Logout</DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
    */
  }
};

export { DashUser };
