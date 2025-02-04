import { LucideContact, LucideScrollText, LucideWrench } from "lucide-react";
import { GrainEffect } from "../GrainEffect";
import { Button } from "../ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "../ui/dropdown-menu";
import { CopyAuthorID } from "./CopyID";
import { Option, Some } from "@/lib/option";
import { User } from "@/types/users";
import { DeleteAuthorButton } from "./DeleteAuthorButton";

type AuthorData = {
  id: string;
  fullname: string;
  codename: string;
};

type ExtendedAuthorData = {
  author: AuthorData;
  quote_count: number;
  line_count: number;
};

const Author = (props: {
  authordata: ExtendedAuthorData;
  userdata: Option<User>;
}) => {
  return (
    <div className="p-3 pt-4 w-80 bg-sidebar border rounded-md backdrop-blur relative overflow-clip">
      <GrainEffect />
      <LucideContact
        className="bottom-6 right-6 -rotate-12 absolute opacity-[.05] scale-[4.5] scale-y-[4.25] z-10"
        aria-hidden
      />
      <h2 className="text-xl text-center font-semibold">
        {props.authordata.author.fullname}
      </h2>
      <p className="text-sm text-center text-gray-500 italic">
        {props.authordata.author.codename}
      </p>
      <hr className="my-4" />
      <div className="flex flex-row justify-around">
        <div className="text-center">
          <p className="uppercase font-semibold text-middleground">
            {"Quotes"}
          </p>
          <h3 className="text-2xl">{props.authordata.quote_count}</h3>
        </div>
        <div className="text-center">
          <p className="uppercase font-semibold text-middleground">{"Lines"}</p>
          <h3 className="text-2xl">{props.authordata.line_count}</h3>
        </div>
      </div>
      <div className="flex flex-row mt-4 gap-3">
        <Button variant={"outline"} className="w-full z-20" disabled>
          <LucideScrollText className="!size-5" />
          {"Quotes"}
        </Button>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant={"outline"} className="w-full z-20">
              <LucideWrench className="!size-5" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent>
            <CopyAuthorID id={props.authordata.author.id} />
            {props.userdata instanceof Some && (
              <>
                <DropdownMenuSeparator />
                {/* <ModifyAuthorButton id={props.authordata.author.id} />
                <DropdownMenuSeparator /> */}
                <DeleteAuthorButton id={props.authordata.author.id} />
              </>
            )}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  );
};

export { Author };
export type { AuthorData, ExtendedAuthorData };
