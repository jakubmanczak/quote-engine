import { Dashboard } from "@/components/Dashboard";
import { qfetch } from "@/lib/qfetch";
import { cookies } from "next/headers";

export default async function LogsPage() {
  // THIS DOESN'T REVALIDATE ON LOGOUT
  // OR LIKE EVER, ACTUALLY
  // TODO: FIX THIS
  const cookiestore = cookies();
  const qauth = cookiestore.get("qauth")?.value;
  const res = await qfetch("/logs?limit=200&page=1", {
    cache: "no-store",
    headers: {
      Cookie: `qauth=${qauth}`,
      "Cache-Control": "no-store",
    },
  });
  const logs: {
    id: string;
    timestamp: number;
    actor: string;
    subject: string;
    action: { [actionType: string]: unknown } | string;
  }[] = res.ok && (await res.json());
  return (
    <Dashboard>
      <p className="text-xl">Logs</p>
      {res.ok &&
        logs.map((log) => {
          const [actionType, actionDetails] = Object.entries(log.action)[0];
          return (
            <div key={log.id}>
              {typeof log.action === "string" ? log.action : actionType} <br />
              {typeof log.action !== "string" && (
                <pre className="bg-black/5 p-4 rounded-lg text-[12px]">
                  {JSON.stringify(actionDetails, null, 2)}
                </pre>
              )}
            </div>
          );
        })}
      {!res.ok && "Could not fetch logs."}
    </Dashboard>
  );
}
