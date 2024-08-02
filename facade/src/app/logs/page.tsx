import { Dashboard } from "@/components/Dashboard";

export default function LogsPage() {
  // const res = await fetch("localhost:2019/users", {
  //   cache: "no-cache",
  //   headers: {},
  // });
  // const logs = await res.json();
  return (
    <Dashboard>
      <p className="text-xl">Logs</p>
      {/* {logs.} */}
    </Dashboard>
  );
}
