import { Author, ExtendedAuthorData } from "@/components/author/Author";
import { NewAuthor } from "@/components/author/New";
import { Sidenav } from "@/components/Sidenav";
import { qfetch } from "@/lib/utils";
import { cookies } from "next/headers";

export const dynamic = "force-dynamic";
export default async function AuthorsPage() {
  const res = await qfetch("/authors/extended", {
    headers: {
      Cookie: (await cookies()).toString(),
    },
  });
  const ok = res.ok;
  const data = await res.json();
  return (
    <Sidenav>
      <h1 className="text-5xl font-fancy">{"Authors"}</h1>
      {ok && (
        <>
          <div className="flex flex-row flex-wrap gap-4">
            {data.map((el: ExtendedAuthorData) => {
              return <Author data={el} key={el.author.id} />;
            })}
            <NewAuthor />
          </div>
        </>
      )}
    </Sidenav>
  );
}
