import { Author, ExtendedAuthorData } from "@/components/author/Author";
import { NewAuthor } from "@/components/author/New";
import { Sidenav } from "@/components/Sidenav";
import { qfetch } from "@/lib/utils";
import { User } from "@/types/users";
import { cookies } from "next/headers";
import { Option, Some, None } from "@/lib/option";

export const dynamic = "force-dynamic";
export default async function AuthorsPage() {
  const res_authors = await qfetch("/authors/extended", {
    headers: {
      Cookie: (await cookies()).toString(),
    },
  });
  const res_authme = await qfetch("/users/me", {
    headers: {
      Cookie: (await cookies()).toString(),
    },
  });

  let data_authors: Option<ExtendedAuthorData[]> = new None();
  let data_authme: Option<User> = new None();
  if (res_authors.ok) {
    data_authors = new Some(await res_authors.json());
  }
  if (res_authme.ok) {
    data_authme = new Some(await res_authme.json());
  }
  return (
    <Sidenav>
      <h1 className="text-5xl font-fancy">{"Authors"}</h1>
      {data_authors instanceof Some && (
        <>
          <div className="flex flex-row flex-wrap gap-4">
            {data_authors.value.map((el: ExtendedAuthorData) => {
              return (
                <Author
                  authordata={el}
                  userdata={data_authme}
                  key={el.author.id}
                />
              );
            })}
            <NewAuthor />
          </div>
        </>
      )}
      {!res_authors.ok && (
        <div className="text-2xl font-fancy">
          {"Are you sure you're logged in?"}
        </div>
      )}
    </Sidenav>
  );
}
