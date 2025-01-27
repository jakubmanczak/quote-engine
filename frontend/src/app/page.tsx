// import { DottedEffect } from "@/components/DottedEffect";
import { Quote } from "@/components/Quote";
import { Sidenav } from "@/components/Sidenav";
import { qfetch } from "@/lib/utils";

export const dynamic = "force-dynamic";
export default async function Home() {
  const res = await qfetch("/quotes/randompublic");
  const data = await res.json();
  return (
    <Sidenav>
      <h1 className="text-center text-5xl font-fancy">
        {"Keep note of what people say"}
      </h1>
      {/* <DottedEffect /> */}
      {/*  */}
      <div className="mx-auto max-w-2xl w-full">
        <Quote data={data} />
      </div>
    </Sidenav>
  );
}
