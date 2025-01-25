import { GrainEffect } from "@/components/GrainEffect";
import { Button } from "@/components/ui/button";
import Link from "next/link";

export default function NotFound() {
  return (
    <>
      {/*  */}
      <div className="relative p-4 max-w-xl flex flex-col w-full self-center bg-background rounded-lg border border-middleground">
        <GrainEffect />
        <h1 className="text-center text-4xl font-fancy z-10">
          {"This page doesn't exist."}
        </h1>
        <p className="text-center z-10">
          {"Click the button below to go back to safety."}
        </p>
        <Link href="/" className="mx-auto z-10 mt-8">
          <Button variant={"default"}>{"Homepage."}</Button>
          {/* <button className="mt-8 min-w-36 bg-neutral-200 text-background p-2 rounded-md font-medium border border-middleground border-b-4 border-b-middleground">
            {"Homepage."}
          </button> */}
        </Link>
      </div>
    </>
  );
}
