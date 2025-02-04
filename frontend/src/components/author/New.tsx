import { LucideCirclePlus, LucideContact } from "lucide-react";

export const NewAuthor = () => {
  return (
    <div className="w-80 min-h-48 rounded-md border-2 border-sidebar border-dashed flex justify-center items-center hover:bg-sidebar cursor-pointer overflow-clip relative">
      <LucideCirclePlus className="size-8" />
      <LucideContact
        className="bottom-6 right-6 -rotate-12 absolute opacity-[.05] scale-[4.5] scale-y-[4.25] z-10"
        aria-hidden
      />
    </div>
  );
};
