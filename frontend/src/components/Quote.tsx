import { LucideLock, LucideQuote } from "lucide-react";
import { GrainEffect } from "./GrainEffect";

type QuoteData = {
  id: string;
  lines: {
    id: string;
    content: string;
    author_id: string;
  }[];
  authors: {
    [key: string]: {
      id: string;
      fullname: string;
      codename: string;
    };
  };
  context?: string;
  timestamp: string;
  clearance: number;
  //
  likes?: number;
};

const ClearanceLevel = (props: { level: number }) => {
  const color = `hsl(${((255 - props.level) / 255) * 100}, 45%, 50%)`;
  return (
    <div className="rounded-full px-3 flex flex-row justify-center items-center gap-2 py-1">
      <LucideLock
        className="size-[16px]"
        style={{
          color: color,
        }}
      />
      {props.level}
    </div>
  );
};

// const LikesCounter = (props: { likesnumber: number; liked?: boolean }) => {
//   return (
//     <div className="rounded-full px-3 flex flex-row justify-center items-center gap-2 hover:bg-half-transparent cursor-pointer transition py-1">
//       <LucideHeart
//         className={`size-[16px] ${props.liked && "fill-pink-600 text-pink-600"}`}
//       />
//       <span>{props.likesnumber}</span>
//     </div>
//   );
// };

const Quote = (props: { data: QuoteData }) => {
  return (
    <div className="p-4 bg-sidebar border rounded-md backdrop-blur relative overflow-clip">
      <GrainEffect />
      <LucideQuote
        className="top-4 right-6 -rotate-12 absolute opacity-[.05] scale-[4.5] scale-y-[4.25]"
        aria-hidden
      />
      {props.data.lines.map((line, index) => {
        const showAuthor =
          index === props.data.lines.length - 1 ||
          props.data.lines[index + 1].author_id !== line.author_id;
        return (
          <div key={`${line.id}/${index}`} className="mb-2">
            <span className="flex flex-row gap-2 relative">
              <LucideQuote className="scale-[.65] scale-y-[.50] mt-[6px] absolute opacity-[.3]" />
              <p className="font-fancy text-2xl ml-6">{line.content}</p>
            </span>
            {showAuthor && (
              <p className="text-sm italic ml-3 flex flex-row gap-[6px]">
                <span>{"—"}</span>
                {props.data.authors[line.author_id].codename}
              </p>
            )}
          </div>
        );
      })}
      <div className="flex flex-row mt-6 text-sm items-center">
        {props.data.timestamp.replace("T", " ")}
        <span className="ml-2">{"⋅"}</span>
        <ClearanceLevel level={props.data.clearance} />
        {/* <span>{"⋅"}</span> */}
        {/* <LikesCounter likesnumber={props.data.likes || 0} /> */}
        {props.data.context && <span className="mr-2">{"⋅"}</span>}
        <span className="italic">{props.data.context}</span>
      </div>
    </div>
  );
};

export { Quote };
