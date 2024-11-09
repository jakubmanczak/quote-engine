import { Dashboard } from "@/components/Dashboard";
import { qfetch } from "@/lib/qfetch";
import { cookies } from "next/headers";

export default async function LogsPage() {
  const cookiestore = cookies();
  const qauth = cookiestore.get("qauth")?.value;
  const res = await qfetch("/logs?limit=200&page=1", {
    cache: "no-cache",
    headers: {
      Cookie: `qauth=${qauth}`,
    },
  });
  const logs: {
    id: string;
    timestamp: number;
    actor: string;
    subject: string;
    action: { [actionType: string]: unknown };
  }[] = res.ok && (await res.json());
  return (
    <Dashboard>
      <p className="text-xl">Logs</p>
      {res.ok &&
        logs.map((log) => {
          const [actionType, actionDetails] = Object.entries(log.action)[0];
          return (
            <div key={log.id}>
              {actionType} <br />
              {JSON.stringify(actionDetails)}
            </div>
          );
        })}
      {!res.ok && "Could not fetch logs."}
    </Dashboard>
  );
}
