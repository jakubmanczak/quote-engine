import {
  LucideContact,
  LucideFileClock,
  LucideScrollText,
  LucideSquareActivity,
  // LucideTag,
  LucideUsers,
} from "lucide-react";
import Link from "next/link";

const links: {
  name: string;
  path: string;
  icon: React.ReactNode;
}[] = [
  {
    name: "Quotes",
    path: "/quotes",
    icon: <LucideScrollText />,
  },
  // {
  //   name: "Tags",
  //   path: "/tags",
  //   icon: <LucideTag />,
  // },
  {
    name: "Authors",
    path: "/authors",
    icon: <LucideContact />,
  },
  {
    name: "Users",
    path: "/users",
    icon: <LucideUsers />,
  },
  {
    name: "Logs",
    path: "/logs",
    icon: <LucideFileClock />,
  },
  {
    name: "System Health",
    path: "/health",
    icon: <LucideSquareActivity />,
  },
];

const Navigation = (props: { currentPath?: string }) => {
  return (
    <div className="flex flex-row w-full p-4 gap-8 my-4 mx-auto max-w-xl md:max-w-2xl lg:max-w-4xl xl:max-w-5xl 2xl:max-w-7xl items-center">
      <Link href="/">
        <h1 className="text-3xl sm:text-4xl font-fancy">{"Quote Engine"}</h1>
      </Link>
      <div className="flex flex-row gap-4 ml-auto">
        {links.map((link) => {
          return (
            <Link
              key={link.name}
              href={link.path}
              className={`${props.currentPath === link.path && "underline"} cursor-pointer`}
            >
              <span className="hidden md:block">{link.name}</span>
              <span className="block md:hidden">{link.icon}</span>
            </Link>
          );
        })}
      </div>
      {/* <div className="ml-auto">
        <p>{"Sign In"}</p>
      </div> */}
    </div>
  );
};

export { Navigation };
